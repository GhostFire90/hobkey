#include "paging.h"
#include <stdint.h>
#include <stdbool.h>
#include "limine_requests.h"
#include "PMM.h"
#include "string.h"

#define PAGE_SIZE 4096


typedef enum {LAYER_PML4, LAYER_PDPT, LAYER_PDT, LAYER_PT} map_layer_t;



extern const unsigned char MAXPHYBIT;
extern const unsigned char MAXVRTBIT;
extern void* _END_KERNEL;

static uint64_t phy_mask;
static uint64_t hhdm_offset;
static uint64_t* pml4_location;
static bool my_paging;

extern uint64_t create_mask(uint8_t max_bit);


uint64_t cannonize(uint64_t x, uint64_t n){
    uint64_t sign_bit = 1ll << (n-1);

    if(x & sign_bit){
        return x | (~((1ll<<n)-1));
    }
    else{
        return x | ((1ll<<n)-1);
    }
}


void set_cr3(uint64_t pml4){

    asm volatile(
        "mov %0, %%cr3" 
        :
        :"r"(pml4)
    );

}

void set_pointer(uint64_t* entry, uint64_t physical_address, uint64_t flags){
    *entry &= ~phy_mask;
    *entry |= (physical_address&phy_mask) | flags;
}
uint64_t get_pointer(uint64_t entry){
    return entry&phy_mask;
}
void set_flags(uint64_t* entry, uint64_t flags){
    *entry &= phy_mask;
    *entry |= flags;
}

void get_indexes(uintptr_t addr, uint16_t indexes[4], uint16_t* offset){
    indexes[0] = (addr >> 39) & 0x1ff;
    indexes[1] = (addr >> 30) & 0x1ff;
    indexes[2] = (addr >> 21) & 0x1ff;
    indexes[3] = (addr >> 12) & 0x1ff;
    if(offset)
        *offset = addr & 0xeff;

}


void check_and_alloc(uint64_t* entry, uint64_t flags){

    bool present = (*entry) & PAGING_PRESENT;
    if(!present){

        if(my_paging){
            
        }
        else{
            char* page = get_page();
            
            allocate_page(page+hhdm_offset);
            memset(page+hhdm_offset, 0, PAGE_SIZE);

            set_pointer(entry, (uint64_t)page, flags);
            //*entry = cannonize(*entry, MAXVRTBIT);

        }

    }

}


/// @brief Crawls the current virtual map to find the layer specified by a given virtual address
/// @param virtual_address the virtual address to use 
/// @param layer Say you want whatever entry in the PML4 that the address references, give LAYER_PML4
/// @return Pointer to that entry to modify as you please
uint64_t* map_crawl(uintptr_t virtual_address, map_layer_t layer){
    uint16_t indexes[4] = {0};
    get_indexes(virtual_address, indexes, 0);
    
    if(my_paging){
        // do temp map stuff to get the end result!
        return 0;
    }
    else{
        uint64_t* ret = pml4_location;

        for(int i = 0; i < layer; i++){
            ret = (uint64_t*)(get_pointer(ret[indexes[i]])+hhdm_offset);
        }
        return &ret[indexes[layer]];
    }
}

// See map_crawl for general breif
// this function also calls "check_alloc "
uint64_t* map_crawl_mark(uintptr_t virtual_address, map_layer_t layer, uint64_t flags){
    uint16_t indexes[4] = {0};
    get_indexes(virtual_address, indexes, 0);
    
    if(my_paging){
        // do temp map stuff to get the end result!
        return 0;
    }
    else{
        uint64_t* ret = pml4_location;

        for(int i = 0; i < layer; i++){
            uint64_t* current = &ret[indexes[i]];
            if(flags & PAGING_PRESENT){
                check_and_alloc(current, flags);
            }
            else{
                set_flags(current, flags);
            }
            ret = (uint64_t*)(get_pointer(*current)+hhdm_offset);
        }
        return &ret[indexes[layer]];
    }
}


uintptr_t from_indexes(uint16_t indexes[4], uint16_t offset){
    uintptr_t ret = 0;
    ret |= ((uint64_t)indexes[0] << 39l);
    ret |= ((uint64_t)indexes[1] << 30l);
    ret |= ((uint64_t)indexes[2] << 21l);
    ret |= ((uint64_t)indexes[3] >> 12l);
    ret += offset;
    ret = cannonize(ret, MAXVRTBIT);
    return ret;
}

// 
void initialize_paging(){
    // setup mask for ONLY the physical address, 11:M-1
    phy_mask = create_mask(MAXPHYBIT-1) ^ create_mask(11);
    // cache limines hhdm offset for ease of use
    hhdm_offset = limine_hhdm()->offset;
    // this bool should get switched to true after completing initial paging setup
    my_paging = false;    


    // get a page from PMM and allocate it
    char* PML4 = (char*)get_page()+hhdm_offset;
    allocate_page(PML4);
    pml4_location = (uint64_t*)PML4;
    // clear that bad boi
    memset(PML4, 0, PAGE_SIZE);

    const struct limine_kernel_address_response* ka = limine_kernel_addr();

    // mapping the kernel correctly so the program counter doesnt get lost 😦
    uint64_t end_kernel = (uint64_t)&_END_KERNEL;
    uint64_t current = ka->virtual_base;
    uint64_t current_phy = ka->physical_base;

    while(current < end_kernel){
        uint64_t* pte =  map_crawl_mark(current, LAYER_PT, PAGING_PRESENT | PAGING_RW);
        set_pointer(pte, current_phy, PAGING_PRESENT | PAGING_RW);
        current += PAGE_SIZE;
        current_phy += PAGE_SIZE;
    }

    // start those damn engines 😆
    set_cr3((uint64_t)PML4-hhdm_offset);    
    
}
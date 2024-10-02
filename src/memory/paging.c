#include "paging.h"
#include <stdint.h>
#include <stdbool.h>
#include <helpers/limine_requests.h>
#include <memory/PMM.h>
#include <helpers/memops.h>

#define PAGE_SIZE 4096



extern const unsigned char MAXPHYBIT;
extern const unsigned char MAXVRTBIT;
extern const uint64_t STACK_ADDRESS;
extern void* _END_KERNEL;

static uint64_t phy_mask;
static uint64_t hhdm_offset;
static uint64_t* pml4_location;
static bool my_paging;
static uint64_t* temp_map_entry;
static uint64_t* temp_map_memory;
//static uint64_t temp_map_index;


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

extern void invalidate_page(uint64_t* vadder);

void set_pointer(uint64_t* entry, uint64_t physical_address, uint64_t flags){
    //*entry &= ~phy_mask;
    *entry = (physical_address&phy_mask) | flags;
}
uint64_t get_pointer(uint64_t entry){
    return entry&phy_mask;
}
void set_flags(uint64_t* entry, uint64_t flags){
    uint64_t e = *entry;
    e &= phy_mask;
    e |= flags;
    *entry = e;
}

void get_indexes(uintptr_t addr, uint16_t indexes[4], uint16_t* offset){
    indexes[0] = (addr >> 39) & 0x1ff;
    indexes[1] = (addr >> 30) & 0x1ff;
    indexes[2] = (addr >> 21) & 0x1ff;
    indexes[3] = (addr >> 12) & 0x1ff;
    if(offset)
        *offset = addr & 0xeff;

}

void *map_to_temp(void *addr)
{
    set_pointer(temp_map_entry, (uint64_t)addr, PAGING_PRESENT | PAGING_RW);
    invalidate_page(temp_map_memory);
    return temp_map_memory;
}

void *map_temp_nearest(void *addr)
{
    uint64_t current = (uint64_t)addr;
    uint64_t offset = current%4096;
    uint64_t page = current-offset;
    char* temp = map_to_temp((void*)page);
    return temp+offset;
}

unsigned long long get_temp()
{
    return get_pointer(*temp_map_entry);
}

void unmap_temp()
{
    set_pointer(temp_map_entry, 0, 0);
    invalidate_page(temp_map_memory);
}

void check_and_alloc(uint64_t* entry, uint64_t flags){

    bool present = (*entry) & PAGING_PRESENT;
    if(!present){

        char* page = get_page();
        
        if(my_paging){
            uint64_t old = get_temp();
            uint64_t* vadder = map_to_temp(page);
            allocate_page(vadder);
            //*vadder = 0xdeadbeef;
            memset(vadder, 0, PAGE_SIZE);
            map_to_temp((void*)old);
        }
        else{
            allocate_page(page+hhdm_offset);
            memset(page+hhdm_offset, 0, PAGE_SIZE);
            
        }
        //void* vadder = my_paging ? map_to_temp(page) : page+hhdm_offset;


        set_pointer(entry, (uint64_t)page, flags);
        //*entry = cannonize(*entry, MAXVRTBIT);
    }
    else{
        set_flags(entry, flags);
    }
}


/// @brief Crawls the current virtual map to find the layer specified by a given virtual address
/// @param virtual_address the virtual address to use 
/// @param layer Say you want whatever entry in the PML4 that the address references, give LAYER_PML4
/// @return Pointer to that entry to modify as you please
uint64_t* map_crawl(uintptr_t virtual_address, map_layer_t layer){
    uint16_t indexes[4] = {0};
    get_indexes(virtual_address, indexes, 0);
    
    
    uint64_t* ret = pml4_location;

    for(int i = 0; i < layer; i++){
        uint64_t phy = get_pointer(ret[indexes[i]]);
        ret = (uint64_t*)(my_paging ? (uint64_t)map_to_temp((void*)phy) : phy + hhdm_offset);
    }
    return &ret[indexes[layer]];

}

// See map_crawl for general breif
// this function also calls "check_alloc "
uint64_t* map_crawl_mark(uintptr_t virtual_address, map_layer_t layer, uint64_t flags){
    uint16_t indexes[4] = {0};
    get_indexes(virtual_address, indexes, 0);
    

    uint64_t* ret = pml4_location;

    for(int i = 0; i < layer; i++){
        uint64_t* current = &ret[indexes[i]];
        if(flags & PAGING_PRESENT){
            check_and_alloc(current, flags);
        }
        else{
            set_flags(current, flags);
        }
        uint64_t phy = get_pointer(*current);
        ret = (uint64_t*)( my_paging ? (uint64_t)map_to_temp((void*)phy) : phy+hhdm_offset);
    }
    return &ret[indexes[layer]];
    
}


uintptr_t from_indexes(uint16_t indexes[4], uint16_t offset){
    uintptr_t ret = 0;
    ret |= ((uint64_t)indexes[0] << 39l);
    ret |= ((uint64_t)indexes[1] << 30l);
    ret |= ((uint64_t)indexes[2] << 21l);
    ret |= ((uint64_t)indexes[3] << 12l);
    ret += offset;
    ret = cannonize(ret, MAXVRTBIT);
    return ret;
}

//UNUSED
uint64_t* index_entry(uint64_t entry, uint16_t index){
    if(my_paging){
        return 0;
    }
    else{

        return (uint64_t*)(get_pointer(entry)+hhdm_offset);
    }
}

// 
unsigned long long initialize_paging(){
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


    // remap limines stack
    uint64_t current = STACK_ADDRESS;
    while(STACK_ADDRESS-current != 0x10000){
        uint64_t* pte = map_crawl_mark((uintptr_t)current, LAYER_PT, PAGING_PRESENT | PAGING_RW);
        set_pointer(pte, current-hhdm_offset, PAGING_RW | PAGING_PRESENT);
        current-=PAGE_SIZE;
    }

    // mapping the kernel correctly so the program counter doesnt get lost 😦
    const struct limine_kernel_address_response* ka = limine_kernel_addr();
    uint64_t end_kernel = (uint64_t)&_END_KERNEL;
    current = ka->virtual_base;
    uint64_t current_phy = ka->physical_base;

    while(current < end_kernel){
        uint64_t* pte =  map_crawl_mark(current, LAYER_PT, PAGING_PRESENT | PAGING_RW);
        set_pointer(pte, current_phy, PAGING_PRESENT | PAGING_RW);
        current += PAGE_SIZE;
        current_phy += PAGE_SIZE;
    }

    // remap pml4 correctly
    uint64_t* pml4_pte = map_crawl_mark((uint64_t)pml4_location, LAYER_PT, PAGING_PRESENT | PAGING_RW);
    set_pointer(pml4_pte, (uint64_t)pml4_location-hhdm_offset, PAGING_PRESENT | PAGING_RW);

    ///hooo boy this part is confusing T-T

    // Some arbitrarily FAR address in the heigher half, last 3 pdt's used
    uint16_t tmp_indexes[4] = {0};

    get_indexes(current, tmp_indexes, 0);
    tmp_indexes[3]++;
    
    temp_map_entry = (uint64_t*)from_indexes(tmp_indexes, 0);
    tmp_indexes[3]++;
   
    //tmp_indexes[2]++;
    temp_map_memory = (uint64_t*)from_indexes(tmp_indexes, 0);

    // initialize the memory location all the way down
    uint64_t* temp_map_pte = map_crawl_mark((uintptr_t)temp_map_memory, LAYER_PT, PAGING_PRESENT | PAGING_RW);
    set_pointer(temp_map_pte, 0, 0);
    // get the pdt we just set up
    uint64_t* temp_map_pdt = map_crawl((uintptr_t)temp_map_memory, LAYER_PDT);

    // get the entries spot initialzed
    uint64_t* temp_map_entry_pte = map_crawl_mark((uintptr_t)temp_map_entry, LAYER_PT, PAGING_PRESENT | PAGING_RW);
    // set that mofo
    set_pointer(temp_map_entry_pte, get_pointer(*temp_map_pdt), PAGING_RW | PAGING_PRESENT);
    temp_map_entry += tmp_indexes[3];
    tmp_indexes[3]++;

    // 🙏

    

    // // framebuffer reloacated to the last of those 3 pdts
    // tmp_indexes[2]++;

    // uint64_t fb_vrt = from_indexes(tmp_indexes, 0);
    // const struct limine_framebuffer_response* fb = limine_framebuffer();
    // uint64_t fb_phy = (uint64_t)fb->framebuffers[0]->address - hhdm_offset;
    // // 4 bytes per pixel
    // uint64_t fb_size = fb->framebuffers[0]->width * fb->framebuffers[0]->height * 4;
    // uint64_t fb_pages = fb_size / PAGE_SIZE + (fb_size % PAGE_SIZE != 0 ? 1 : 0); 

    set_cr3((uint64_t)PML4-hhdm_offset);
    // // make sure the flag is set so we dont like, explode by using old paging
    my_paging = true;
    return from_indexes(tmp_indexes, 0);
    
    // // easier mapping! 
    // for(uint64_t i = 0; i < fb_pages; i++){
    //     map_phy_to_vrt((void*)fb_vrt, (void*)fb_phy, PAGING_PRESENT | PAGING_RW);
    //     fb_phy += PAGE_SIZE;
    //     fb_vrt += PAGE_SIZE;
    // }
    // fb_vrt -= PAGE_SIZE * fb_pages;

    // //testing the color setting
    // memset((void*)fb_vrt, 0xFF, fb_pages*PAGE_SIZE);


    // asm volatile("nop");
    
}





bool CustomPagingEnabled(void)
{
    return my_paging;
}

#include "paging.h"
#include "PMM.h"
#include "string.h"
#include <limine.h>
#include <stdint.h>
#include "limine_requests.h"
#include "terminal.h"
#define PAGE_SIZE 4096
#define ELEVEN_MASK 0xFFFFFFFFFFFFF800

extern const unsigned char MAXPHYBIT;
extern const unsigned char MAXVRTBIT;

extern uint64_t create_mask(uint64_t max_bit);
void set_cr3(void* pml4){

    asm volatile(
        "mov %0, %%cr3" 
        :
        :"r"(pml4)
    );

}

extern void* _END_KERNEL;


static uint64_t hhdm_offset;
static uint64_t mask;
static uint64_t* temp_map;
static uint64_t* temp_location;

uint64_t cannonize(uint64_t x, uint64_t n){
    uint64_t sign_bit = 1ll << (n-1);

    if(x & sign_bit){
        return x | (~((1ll<<n)-1));
    }
    else{
        return x | ((1ll<<n)-1);
    }
}

void set_pointer(uint64_t* entry, uint64_t pointer, uint64_t flags){
    *entry |= (pointer&mask) | flags;
}
uint64_t get_pointer(uint64_t entry){
    return entry&ELEVEN_MASK;
}


//layer 1
void fill_pt(uint64_t* pt){
    for(uint32_t i = 0; i < 512; i++){
        pt[i] = PAGING_RW;
    }
}

//layer 2
void fill_pdt(uint64_t* pdt){
    

    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = 0;
        char* pg = get_page();
        allocate_page(pg+hhdm_offset);
        set_pointer(&entry, (uint64_t)pg, PAGING_RW | PAGING_PRESENT);
        fill_pt((uint64_t*)(pg+hhdm_offset));
        pdt[i] = entry;
    }
}


//layer 3
void fill_pdpt(uint64_t* pdpt){
    
    int free_pages = page_count();
    printf("Pages before: %d\n", free_pages);
    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = 0;
        char* pg = get_page();
        allocate_page(pg+hhdm_offset);
        set_pointer(&entry, (uint64_t)pg, PAGING_RW | PAGING_PRESENT);
        fill_pdt((uint64_t*)(pg+hhdm_offset));
        pdpt[i] = entry;

    }
    free_pages = page_count();
    printf("Pages after: %d\n", free_pages);

}

void get_indexes(uint64_t addr, uint16_t indexes[4], uint16_t* offset){
    indexes[0] = (addr >> 39) & 0x1ff;
    indexes[1] = (addr >> 30) & 0x1ff;
    indexes[2] = (addr >> 21) & 0x1ff;
    indexes[3] = (addr>>12) & 0x1ff;
    *offset = addr & 0xeff;

}
uint64_t from_indexes(uint16_t indexes[4], uint16_t offset){
    uint64_t ret = 0;
    ret |= ((uint64_t)indexes[0] << 39l);
    ret |= ((uint64_t)indexes[1] << 30l);
    ret |= ((uint64_t)indexes[2] << 21l);
    ret |= ((uint64_t)indexes[3] >> 12l);
    ret += offset;
    ret = cannonize(ret, MAXVRTBIT);
    return ret;
}


void* sanity_check(uint64_t vaddr, uint64_t* pml4){
    uint16_t indexes[4] = {0};
    uint16_t page_offset = 0;
    get_indexes(vaddr, indexes, &page_offset);
    uint64_t* pdpt = (uint64_t*)(get_pointer(pml4[indexes[0]])+hhdm_offset);
    uint64_t* pdt = (uint64_t*)(get_pointer(pdpt[indexes[1]])+hhdm_offset);
    uint64_t* pt = (uint64_t*)(get_pointer(pdt[indexes[2]])+hhdm_offset);
    uint64_t ret = get_pointer(pt[indexes[3]]);
    return (void*)ret;

}

void map_to_temp(uint64_t phy_addr){

}


void initialize_paging()
{
    hhdm_offset = limine_hhdm()->offset;

    uint64_t* PML4 = (uint64_t*)((char*)get_page()+hhdm_offset);
    allocate_page(PML4);
    memset(PML4, 0, PAGE_SIZE);
    // uint64_t* PDPT = (uint64_t*)((char*)get_page()+offset);
    // allocate_page(PDPT); 

    // memset(PDPT, 0, PAGE_SIZE);


    PML4[0] = 0x3;

    mask = create_mask(MAXPHYBIT-1);
    //set_pointer(&PML4[0], (uint64_t)PDPT-offset, 0);
    //PML4[0] |= (((uint64_t)PDPT-offset)& mask) << 12L;
    const struct limine_kernel_address_response* ka = limine_kernel_addr();
    uint64_t end_kernel = (uint64_t)&_END_KERNEL;
    uint64_t kernel_size = end_kernel - ka->virtual_base;

    uint64_t kernel_page_count = kernel_size/PAGE_SIZE;


    printf("Page tables initialized, ready for transition\n");
    uint16_t kernel_indexes[4] = {0};
    uint16_t page_offset = 0;
    get_indexes(ka->virtual_base, kernel_indexes, &page_offset);
    //printf("PML4 index = %d\n", indexes[0]);

    PML4[kernel_indexes[0]] = PAGING_PRESENT | PAGING_RW;
    uint64_t* kernel_pdpt = (uint64_t*)((char*)get_page()+hhdm_offset);
    allocate_page(kernel_pdpt);
    set_pointer(&PML4[kernel_indexes[0]], (uint64_t)kernel_pdpt-hhdm_offset, PAGING_PRESENT | PAGING_RW);
    //PML4[indexes[0]] |= (((uint64_t)PDPT-offset) & mask) << 12L;
    fill_pdpt(kernel_pdpt);

    // hate paging
    // pdpte, the entry in the pdpt that the pointer references D:<
    uint64_t* kernel_pdt = (uint64_t*)(get_pointer(kernel_pdpt[kernel_indexes[1]])+hhdm_offset);
    uint64_t* kernel_table = ((uint64_t*)(get_pointer(kernel_pdt[kernel_indexes[2]])+hhdm_offset))+kernel_indexes[3];
    uint64_t phy_addr = ka->physical_base;
    for(uint64_t i = 0; i < kernel_page_count; i++, phy_addr+=PAGE_SIZE){
        set_pointer(&kernel_table[i], phy_addr, PAGING_PRESENT | PAGING_RW);
    }
    uint64_t test = (uint64_t)sanity_check(ka->virtual_base, PML4);
    printf("break me!");

    kernel_indexes[2]+=1;
    uint64_t* temp_table = ((uint64_t*)(get_pointer(kernel_pdt[kernel_indexes[2]])+hhdm_offset))+kernel_indexes[3];
    temp_map = (uint64_t*)from_indexes(kernel_indexes, 0);
    
    kernel_indexes[2]+=1;
    uint64_t* tmp_ptr = ((uint64_t*)(get_pointer(kernel_pdt[kernel_indexes[2]])+hhdm_offset))+kernel_indexes[3];
    set_pointer(temp_table, ((uint64_t) tmp_ptr)-hhdm_offset, PAGING_RW | PAGING_PRESENT);
    temp_location = (uint64_t*)from_indexes(kernel_indexes, 0);


    set_cr3(((char*)PML4)-hhdm_offset);

    memset(temp_map, 0xff, 8);

    asm volatile ("nop");

}
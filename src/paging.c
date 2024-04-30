#include "paging.h"
#include "PMM.h"
#include "string.h"
#include <limine.h>
#include <stdint.h>
#include "limine_requests.h"
#include "terminal.h"
#define PAGE_SIZE 4096

extern const unsigned char MAXPHYBIT;

extern uint64_t create_mask(uint64_t max_bit);
extern void set_cr3(void* pml4);

extern void* _END_KERNEL;

static const uint64_t base = 0x3;
static uint64_t offset;
static uint64_t mask;

//layer 1
void fill_pt(uint64_t* pt){
    for(uint32_t i = 0; i < 512; i++){
        pt[i] = 2lu;
    }
}

//layer 2
void fill_pdt(uint64_t* pdt){
    

    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = base;
        char* pg = get_page();
        allocate_page(pg+offset);
        entry |= ((uint64_t)entry & mask) << 12;
        fill_pt((uint64_t*)(pg+offset));
        pdt[i] = entry;
    }
}


//layer 3
void fill_pdpt(uint64_t* pdpt){
    

    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = base;
        char* pg = get_page();
        allocate_page(pg+offset);
        entry |= ((uint64_t)entry & mask) << 12;
        fill_pdt((uint64_t*)(pg+offset));
        pdpt[i] = entry;

    }
}

void get_indexes(uint64_t addr, uint16_t indexes[4], uint16_t* offset){
    indexes[0] = (addr >> 39) & 0x1ff;
    indexes[1] = (addr >> 30) & 0x1ff;
    indexes[2] = (addr >> 21) & 0x1ff;
    indexes[3] = (addr>>12) & 0x1ff;
    *offset = addr & 0xeff;

}


void initialize_paging()
{
    offset = limine_hhdm()->offset;

    uint64_t* PML4 = (uint64_t*)((char*)get_page()+offset);
    allocate_page(PML4);
    uint64_t* PDPT = (uint64_t*)((char*)get_page()+offset);
    allocate_page(PDPT); 

    memset(PML4, 0, PAGE_SIZE);
    memset(PDPT, 0, PAGE_SIZE);

    PML4[0] = 0x3;

    mask = create_mask(MAXPHYBIT-1);
    PML4[0] |= (((uint64_t)PDPT-offset)& mask) << 12L;
    fill_pdpt(PDPT);
    const struct limine_kernel_address_response* ka = limine_kernel_addr();
    uint64_t end_kernel = (uint64_t)&_END_KERNEL;
    uint64_t kernel_size = end_kernel - ka->virtual_base;


    printf("Page tables initialized, ready for transition\n");
    uint16_t indexes[4] = {0};
    uint16_t page_offset = 0;
    get_indexes(ka->virtual_base, indexes, &page_offset);
    printf("PML4 index = %d\n", indexes[0]);

    PML4[indexes[0]] = 0x3;
    uint64_t* kernel_pdpt = (uint64_t*)((char*)get_page()+offset);
    allocate_page(kernel_pdpt);
    PML4[indexes[0]] |= (((uint64_t)PDPT-offset) & mask) << 12L;
    fill_pdpt(kernel_pdpt);
    


    //set_cr3((char*)PML4-offset);
    


    




}
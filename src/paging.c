#include "paging.h"
#include "PMM.h"
#include "string.h"
#include <limine.h>
#include <stdint.h>
#include "limine_requests.h"
#include "terminal.h"

extern const unsigned char MAXPHYBIT;

extern uint64_t create_mask(uint64_t max_bit);
extern void set_cr3(void* pml4);

extern void* _END_KERNEL;

//layer 1
void fill_pt(uint64_t* pt){
    for(uint32_t i = 0; i < 512; i++){
        pt[i] = 2lu;
    }
}

//layer 2
void fill_pdt(uint64_t* pdt){
    const uint64_t base = 0x3;
    const uint64_t offset = limine_hhdm()->offset;
    const uint64_t mask = create_mask(MAXPHYBIT-1);

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
    const uint64_t base = 0x3;
    const uint64_t offset = limine_hhdm()->offset;
    const uint64_t mask = create_mask(MAXPHYBIT-1);

    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = base;
        char* pg = get_page();
        allocate_page(pg+offset);
        entry |= ((uint64_t)entry & mask) << 12;
        fill_pdt((uint64_t*)(pg+offset));
        pdpt[i] = entry;

    }
}

void get_indexes(uint64_t addr, uint8_t indexes[4], uint16_t* offset){
    indexes[0] = addr >> 39;
    indexes[1] = (addr >> 30) & 0xff;
    indexes[2] = (addr >> 21) & 0xff;
    indexes[3] = (addr>>12) & 0xff;
    *offset = addr & 0xff;

}


void initialize_paging()
{
    uint64_t offset = limine_hhdm()->offset;

    uint64_t* PML4 = (uint64_t*)((char*)get_page()+offset);
    allocate_page(PML4);
    uint64_t* PDPT = (uint64_t*)((char*)get_page()+offset);
    allocate_page(PDPT); 

    memset(PML4, 0, 4096);
    memset(PDPT, 0, 4096);

    PML4[0] = 0x3;

    uint64_t mask = create_mask(MAXPHYBIT-1);
    PML4[0] |= (((uint64_t)PDPT-offset)& mask) << 12L;
    fill_pdpt(PDPT);
    const struct limine_kernel_address_response* ka = limine_kernel_addr();
    const void* end_kernel = &_END_KERNEL;
    printf("Page tables initialized, ready for transition");

    //set_cr3((char*)PML4-offset);
    


    




}
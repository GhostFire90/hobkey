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

extern uint64_t create_mask(uint64_t max_bit);
void set_cr3(void* pml4){

    asm volatile(
        "mov %0, %%cr3" 
        :
        :"r"(pml4)
    );

}

extern void* _END_KERNEL;

static const uint64_t base = 0x0;
static uint64_t offset;
static uint64_t mask;

void set_pointer(uint64_t* entry, uint64_t pointer, uint64_t flags){
    *entry |= (pointer&mask) | flags; 
}
uint64_t get_pointer(uint64_t entry){
    return (entry)&ELEVEN_MASK;
}


//layer 1
void fill_pt(uint64_t* pt){
    for(uint32_t i = 0; i < 512; i++){
        pt[i] = base;
    }
}

//layer 2
void fill_pdt(uint64_t* pdt){
    

    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = base;
        char* pg = get_page();
        allocate_page(pg+offset);
        memset(pg+offset, 0, PAGE_SIZE);
        set_pointer(&entry, (uint64_t)pg, PAGING_PRESENT | PAGING_RW);
        fill_pt((uint64_t*)(pg+offset));
        pdt[i] = entry;
    }
}


//layer 3
void fill_pdpt(uint64_t* pdpt){
    
    int free_pages = page_count();
    printf("Pages before: %d\n", free_pages);
    for(uint32_t i = 0; i < 512u; i++){
        uint64_t entry = base;
        char* pg = get_page();
        allocate_page(pg+offset);
        memset(pg+offset, 0, PAGE_SIZE);

        set_pointer(&entry, (uint64_t)pg, PAGING_PRESENT | PAGING_RW);
        fill_pdt((uint64_t*)(pg+offset));
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


void* sanity_check(uint64_t vaddr, uint64_t* pml4){
    uint16_t indexes[4] = {0};
    uint16_t page_offset = 0;
    get_indexes(vaddr, indexes, &page_offset);
    uint64_t* pdpt = (uint64_t*)(get_pointer(pml4[indexes[0]])+offset);
    uint64_t* pdt = (uint64_t*)(get_pointer(pdpt[indexes[1]])+offset);
    uint64_t* pt = (uint64_t*)(get_pointer(pdt[indexes[2]])+offset);
    uint64_t ret = get_pointer(pt[indexes[3]]);
    return (void*)ret;

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

    //PML4[0] = 0x3;

    mask = create_mask(MAXPHYBIT-1);
    //set_pointer(&PML4[0], (uint64_t)PDPT-offset);
    //PML4[0] |= (((uint64_t)PDPT-offset)& mask) << 12L;
    //fill_pdpt(PDPT);
    const struct limine_kernel_address_response* ka = limine_kernel_addr();
    uint64_t end_kernel = (uint64_t)&_END_KERNEL;
    uint64_t kernel_size = end_kernel - ka->virtual_base;

    uint64_t kernel_page_count = kernel_size/PAGE_SIZE;


    printf("Page tables initialized, ready for transition\n");
    uint16_t indexes[4] = {0};
    uint16_t page_offset = 0;
    get_indexes(ka->virtual_base, indexes, &page_offset);
    //printf("PML4 index = %d\n", indexes[0]);

    PML4[indexes[0]] = 0x3;
    uint64_t* kernel_pdpt = (uint64_t*)((char*)get_page()+offset);
    allocate_page(kernel_pdpt);
    set_pointer(&PML4[indexes[0]], (uint64_t)kernel_pdpt-offset, PAGING_PRESENT | PAGING_RW);
    //PML4[indexes[0]] |= (((uint64_t)PDPT-offset) & mask) << 12L;
    fill_pdpt(kernel_pdpt);

    // hate paging
    // pdpte, the entry in the pdpt that the pointer references D:<
    uint64_t* kernel_pdt = (uint64_t*)(get_pointer(kernel_pdpt[indexes[1]])+offset);
    uint64_t* kernel_table = ((uint64_t*)(get_pointer(kernel_pdt[indexes[2]])+offset))+indexes[3];
    uint64_t phy_addr = ka->physical_base;
    for(uint64_t i = 0; i < kernel_page_count; i++, phy_addr+=PAGE_SIZE){
        set_pointer(&kernel_table[i], phy_addr, PAGING_PRESENT | PAGING_RW);
    }
    uint64_t test = (uint64_t)sanity_check(ka->virtual_base, PML4);
    printf("break me!");

    set_cr3(((char*)PML4)-offset);
}
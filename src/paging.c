#include "paging.h"
#include "PMM.h"
#include "string.h"
#include <limine.h>
#include <stdint.h>
#include "limine_requests.h"

extern const unsigned char MAXPHYBIT;

uint64_t create_mask(uint64_t max_bit);

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

    uint64_t mask = create_mask(MAXPHYBIT);
    PML4[0] |= ((((uint64_t)PDPT-offset)>>12)& mask) << 12L;


    




}
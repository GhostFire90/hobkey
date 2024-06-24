#include "virtual_memory_management.h"

#include <stdint.h>
#include "PMM.h"
#include "paging.h"


extern void invalidate_page(uint64_t);
static uint64_t next_kernel_page;

void initailize_vmm(uint64_t _next_kernel_page)
{
    next_kernel_page = _next_kernel_page;
}

void* extend_kernel_map(void* page)
{
    //void* page = get_page();
    void* addr = (void*)next_kernel_page;
    map_phy_to_vrt(addr, page, PAGING_PRESENT | PAGING_RW);

    next_kernel_page+=PAGE_SIZE;
    return addr;
}

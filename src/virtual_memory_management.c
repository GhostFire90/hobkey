#include "virtual_memory_management.h"

#include <stdint.h>
#include "PMM.h"
#include "paging.h"


extern void invalidate_page(void*);
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

void map_phy_to_vrt(void *virtual, void *physical, unsigned long flags)
{
    uint64_t* location = map_crawl_mark((uintptr_t) virtual, LAYER_PT, flags);
    set_pointer(location, (uint64_t) physical, flags);
    invalidate_page(virtual);
    unmap_temp();
    //set_cr3((uint64_t)pml4_location);
}

void unmap_page(void * virtual)
{
    free_page(virtual);
    uint64_t* entry = map_crawl((uint64_t)virtual, LAYER_PT);
    set_pointer(entry, 0, 0);
    invalidate_page(entry);

}

void remap_page(void *old, void *new, uint64_t flags)
{
    uint64_t* val = map_crawl((uint64_t)old, LAYER_PT);
    set_pointer(map_crawl_mark((uint64_t)new, LAYER_PT, flags), *val, flags);
    set_pointer(val, 0, 0);
    invalidate_page(old);
    invalidate_page(new);
    
}

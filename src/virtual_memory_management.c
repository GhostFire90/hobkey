#include "virtual_memory_management.h"

#include <stdint.h>
#include "PMM.h"
#include "paging.h"

extern uint64_t cannonize(uint64_t x, uint64_t n);
extern const unsigned char MAXVRTBIT;
extern void invalidate_page(void*);
static uint64_t next_kernel_page;
static uint64_t next_page = 0x100000000;
static uint64_t pages_allocated = 0;

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

void *vm_allocate_page()
{
    if(next_page >= 0xffffffff80000000){
        return 0;
    }
    void* ret = (void*)next_page;
    next_page+=0x1000;
    pages_allocated++;
    //ret = (void*)cannonize((uint64_t)ret, MAXVRTBIT);
    void* phy_page = get_page();
    allocate_page(map_to_temp(phy_page));
    unmap_temp();
    map_phy_to_vrt(ret, phy_page, PAGING_PRESENT | PAGING_RW);
    //allocate_page(ret);

    return ret;
}

void vm_free_page(void* pg)
{
    unmap_page(pg);
}

void remap_page(void *old, void *new, uint64_t flags)
{
    uint64_t* val = map_crawl((uint64_t)old, LAYER_PT);
    set_pointer(map_crawl_mark((uint64_t)new, LAYER_PT, flags), *val, flags);
    set_pointer(val, 0, 0);
    invalidate_page(old);
    invalidate_page(new);
    
}

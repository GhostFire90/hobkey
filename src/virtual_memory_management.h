#ifndef VMM_H
#define VMM_H

void initailize_vmm(unsigned long next_kernel_page);
void* extend_kernel_map(void* page);
void map_phy_to_vrt(void* virtual, void* physical, unsigned long  flags);
void unmap_page(void* virtual);
void remap_page(void* old, void* new, unsigned long flags);

#endif
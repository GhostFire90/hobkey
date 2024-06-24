#ifndef VMM_H
#define VMM_H

void initailize_vmm(unsigned long next_kernel_page);
void* extend_kernel_map(void* page);

#endif
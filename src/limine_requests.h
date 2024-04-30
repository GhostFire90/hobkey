#ifndef LIM_REQ_H
#define LIM_REQ_H
#include <limine.h>


const struct limine_memmap_response* limine_memmap();
const struct limine_hhdm_response* limine_hhdm();
const struct limine_kernel_address_response* limine_kernel_addr();


#endif
#include <stddef.h>
#include <stdint.h>

#include "boot_param.h"
#include "ramdisc.h"
#include "bmp.h"
#include "string.h"
#include "stdlib.h"
#include "terminal.h"
#include "idt.h"
#include <limine.h>
#include "PMM.h"
#include "paging.h"
#include "limine_requests.h"
#include "virtual_memory_management.h"
LIMINE_BASE_REVISION(1)





#include "psf.h"

const char* message = "Hello";



int32_t kernel_main(void){

    struct limine_framebuffer fb;
    struct limine_file initrd;
    memcpy(&initrd,limine_modules()->modules[0], sizeof(struct limine_file));
    memcpy(&fb, limine_framebuffer()->framebuffers[0], sizeof(struct limine_framebuffer));
    uint64_t hhdm_offset = limine_hhdm()->offset;
    
    
    build_list();
    uint64_t next_kernel_address = initialize_paging();
    initailize_vmm(next_kernel_address);

    void* fb_begin = (void*)next_kernel_address;;
    uint64_t fb_phy = (uint64_t)fb.address-hhdm_offset;
    uint64_t fb_size = fb.width * fb.height * (fb.bpp/8 + (fb.bpp % 8 != 0 ? 1 : 0));
    uint64_t fb_pages = fb_size / PAGE_SIZE + (fb_size % PAGE_SIZE != 0 ? 1 : 0); 

    for(uint64_t i = 0; i < fb_pages; i++){
        extend_kernel_map((void*)fb_phy);
        fb_phy+=PAGE_SIZE;
    }
    fb.address = fb_begin;
    //memset(fb_begin, 0xff, fb_size);

    //i should change this for relocation probably
    void* initrd_begin = (void*)(next_kernel_address+fb_pages*PAGE_SIZE);
    uint64_t initrd_phy = (uint64_t)initrd.address-hhdm_offset;
    uint64_t initrd_pages = initrd.size / PAGE_SIZE + (initrd.size % PAGE_SIZE != 0 ? 1 : 0);
    for(uint64_t i = 0; i < initrd_pages; i++){
        extend_kernel_map((void*)initrd_phy);
        initrd_phy+= PAGE_SIZE;
    }

    InitializeRamdisc((char*)initrd_begin, initrd.size);
    InitializeTerminal(&fb);
    printf("Paging remap complete");    

    while(1){}
    return 0;
}
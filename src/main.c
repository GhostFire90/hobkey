#include <stddef.h>
#include <stdint.h>

#include <helpers/ramdisc.h>
#include <helpers/memops.h>

#include "idt.h"
#include <limine.h>
#include <memory/PMM.h>
#include <memory/paging.h>
#include <helpers/limine_requests.h>
#include <timers/apic.h>
#include <timers/hpet.h>
#include <memory/virtual_memory_management.h>
#include <streams/framebuffer_stream.h>
#include <streams/terminal_stream.h>
#include <helpers/system_tables.h>
#include <memory/liballoc.h>
LIMINE_BASE_REVISION(1)

#include <helpers/psf.h>






int32_t kernel_main(void){

    void* initrd_begin;
    uint64_t initrd_size;
    void* fb_begin;
    stream_t* fb_stream;
    {
        struct limine_framebuffer fb;
        struct limine_file initrd;
        struct limine_rsdp_response rsdp;
        memcpy(&initrd,limine_modules()->modules[0], sizeof(struct limine_file));
        memcpy(&fb, limine_framebuffer()->framebuffers[0], sizeof(struct limine_framebuffer));
        memcpy(&rsdp, limine_rsdp(), sizeof(rsdp));
        uint64_t hhdm_offset = limine_hhdm()->offset;
        initrd_size = initrd.size;
        
        
        
        build_list();
        uint64_t next_kernel_address = initialize_paging();
        initailize_vmm(next_kernel_address);

        // uint64_t rsdp_phy = (uint64_t)rsdp.address-hhdm_offset;
        // uint64_t offset = rsdp_phy % PAGE_SIZE;

        //char* rsdp_temp = map_to_temp((void*)(rsdp_phy-offset));
        //rsdp_temp+=offset;


        
        //find_table("APIC");

        fb_begin = (void*)next_kernel_address;;
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
        initrd_begin = (void*)(next_kernel_address+fb_pages*PAGE_SIZE);
        uint64_t initrd_phy = (uint64_t)initrd.address-hhdm_offset;
        uint64_t initrd_pages = initrd.size / PAGE_SIZE + (initrd.size % PAGE_SIZE != 0 ? 1 : 0);
        for(uint64_t i = 0; i < initrd_pages; i++){
            extend_kernel_map((void*)initrd_phy);
            initrd_phy+= PAGE_SIZE;
        }
        fb_stream = create_framebuffer_stream(&fb);
        initialize_tables(map_temp_nearest((void*)((uint64_t)rsdp.address-hhdm_offset)));
        unmap_temp();
    }
    InitializeRamdisc((char*)initrd_begin, initrd_size);
    stream_t* term_stream = create_terminal_stream(fb_stream);
    apic_initialize();
    hpet_initialize();
    stream_write(term_stream, "hello world\nabcd\n", 17);

    while(1);
    return 0;
}
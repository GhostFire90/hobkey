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

LIMINE_BASE_REVISION(1)

struct limine_framebuffer_request frame_buffer_req = {
    .id = LIMINE_FRAMEBUFFER_REQUEST,
    .revision = 0
};

struct limine_module_request initrd_req = {
    .id = LIMINE_MODULE_REQUEST,
    .revision = 0
};

#include "psf.h"

const char* message = "Hello";


int32_t kernel_main(void){
    InitializeRamdisc(initrd_req.response->modules[0]->address, initrd_req.response->modules[0]->size);
    //memset(bp->frameBuffer, 0x00, (bp->resX*4)*(bp->resY*4));

    struct limine_framebuffer *fb = frame_buffer_req.response->framebuffers[0];
    psf_t font;
    GetPsf(&font, "resources/zap-vga.psf");
    

    InitializeTerminal(fb);
    printf("Hello world");

    while(1){}
    return 0;
}
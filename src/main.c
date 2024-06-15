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
LIMINE_BASE_REVISION(1)





#include "psf.h"

const char* message = "Hello";



int32_t kernel_main(void){
    InitializeRamdisc(limine_modules()->modules[0]->address, limine_modules()->modules[0]->size);
    //memset(bp->frameBuffer, 0x00, (bp->resX*4)*(bp->resY*4));

    struct limine_framebuffer *fb = limine_framebuffer()->framebuffers[0];
    

    InitializeTerminal(fb);

    

    //printf("MAX_PHY_BIT: %d\n", MAXPHYBIT);
    //printf("Paging Mode: %d\n", paging_req.response->mode);
    build_list();
    initialize_paging();
    

    while(1){}
    return 0;
}
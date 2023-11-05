#include <stddef.h>
#include <stdint.h>

#include "boot_param.h"
#include "ramdisc.h"
#include "bmp.h"
#include "helpers.h"
#include "terminal.h"

int32_t kernel_main(struct BootParam* bp){
    //InitRamdisc(bp->ramDisc, bp->ramDiscSize);
    
    memset(bp->frameBuffer, 0x00, (bp->resX*4)*(bp->resY*4));
    
    InitializeTerminal(bp);
    
    //putChar('0');

    printf("hello: %d, %s",42069, "woo lesgo");
    flush();

    while(1);
    return 0;
}
#include <stddef.h>
#include <stdint.h>

#include "boot_param.h"
#include "ramdisc.h"
#include "bmp.h"
#include "string.h"
#include "stdlib.h"
#include "terminal.h"

int32_t kernel_main(struct BootParam* bp){
    //InitRamdisc(bp->ramDisc, bp->ramDiscSize);
    
    memset(bp->frameBuffer, 0x00, (bp->resX*4)*(bp->resY*4));
    
    InitializeTerminal(bp);

    char num[33];
    
    write(num, strlen(num));

    while(1);
    return 0;
}
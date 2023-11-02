#include <stddef.h>
#include <stdint.h>

#include "boot_param.h"
#include "ramdisc.h"
#include "bmp.h"
#include "helpers.h"

int32_t kernel_main(struct BootParam* bp){
    //InitRamdisc(bp->ramDisc, bp->ramDiscSize);
    
    memset(bp->frameBuffer, 0xFF, (bp->resX*4)*(bp->resY*4));
    unsigned long file_size = 0x00;
    char* file = FindFile(bp->ramDisc, bp->ramDiscSize, "resources/font.bmp", &file_size);
    if(file == NULL){
        memset(bp->frameBuffer, 0x00, (bp->resX*4)*(bp->resY*4));
    }
    else{
        unsigned width = *(unsigned *)(file+0x12);
        unsigned height = *(unsigned *)(file+0x16);
        unsigned offset = *(unsigned*)(file+0x0a);
        unsigned short bitDepth = *(unsigned short*)(file+0x1c);

        unsigned rowSize = (bitDepth*width+31)/32;
        rowSize *= 4;
        

        for(unsigned short y = 0; y < height; y++){
            for(unsigned short x = 0; x < rowSize; x++){
                bp->frameBuffer[x+(height-y)*(bp->resX*4)] = file[offset+x+y*rowSize];
            }
        }
    }
    
    while(1);
    return 0;
}
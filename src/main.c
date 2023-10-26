#include <stddef.h>
#include <stdint.h>


int32_t kernel_main(uint32_t* buffer, uint32_t width, uint32_t height, uint64_t pitch){

    for(uint32_t y = 0; y < height; y++){
        for(uint32_t x = 0; x < width; x++){
            uint8_t red = ((float)x/width)*0xff;
            uint8_t green = ((float)y/height)*0xff;
            *(buffer + width*y+x) = (red<<16)+(green<<8);
        }
    }
    //while(1);
    return 0;
}
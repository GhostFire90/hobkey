#include "bmp.h"
#include "memops.h"
#include <stdint.h>
#include <byteswap.h>

struct BMP_HEADER{
    char ident[2];
    uint32_t size;
    uint32_t UNUSED;
    uint32_t startOffset;
};
struct DIB_HEADER{
    uint32_t headerSize;
    uint16_t width, height;
};


void ReadBMP(const char *file, bmp_t *bmp)
{
    //struct BMP_HEADER *bm_h = (struct BMP_HEADER *)file;
    
    bmp->height = *(unsigned*)(file+0x16);
    bmp->width = *(unsigned*)(file+0x12);
    bmp->offset = *(unsigned*)(file+0x0a);
    bmp->bit_depth = *(unsigned short*)(file+0x1c);
    bmp->data = file+bmp->offset;
    bmp->rowSize = ((bmp->bit_depth*bmp->width+31)/32)*4;
}
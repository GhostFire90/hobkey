#include "bmp.h"
#include "helpers.h"
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
    struct BMP_HEADER *bm_h = (struct BMP_HEADER *)file;
    bmp->data = file+bm_h->startOffset;
    
    bmp->height = *(short*)file+0x12;
    bmp->width = *(short*)file+0x14;
}
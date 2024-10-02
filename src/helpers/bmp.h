#ifndef BMP_H
#define BMP_H

typedef struct BMP{
    unsigned width, height;
    unsigned short bit_depth;
    unsigned offset;
    unsigned rowSize;
    const char* data;
} bmp_t;

void ReadBMP(const char* file, bmp_t* bmp);



#endif
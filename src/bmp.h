#ifndef BMP_H
#define BMP_H

typedef struct BMP{
    char* data;
    unsigned short width, height;
} bmp_t;

void ReadBMP(const char* file, bmp_t* bmp);



#endif
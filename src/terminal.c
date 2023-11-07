#include "terminal.h"
#include "boot_param.h"
#include <stdint.h>
#include "bmp.h"
#include "string.h"
#include <stdarg.h>
#include "ramdisc.h"
#include "stdlib.h"

#define CHARACTER_SIZE 6

static char* frameBuffer;
static uint32_t width,height;
static uint32_t x_offset, y_offset;
static bmp_t font;
static uint64_t font_size;



void InitializeTerminal(struct BootParam* bp)
{
    ReadBMP(FindFile(bp->ramDisc, bp->ramDiscSize, "resources/font.bmp", &font_size), &font);
    x_offset = y_offset = 0;
    width = bp->resX;
    height = bp->resY;
    frameBuffer = bp->frameBuffer;
}



void putChar(char c)
{

    uint32_t buff_stride = (font.bit_depth/8)*CHARACTER_SIZE;
    uint32_t frameBuffer_offset = c*buff_stride;

    uint32_t rows_down = frameBuffer_offset/font.rowSize;
    frameBuffer_offset -= font.rowSize*rows_down;
    

    for(unsigned i = 0; i < CHARACTER_SIZE; i++){
        char* fb_pos = (y_offset*width)+x_offset+frameBuffer+(4*width*i);
        const char* font_pos = (frameBuffer_offset)+((font.height-i-1)*font.rowSize)+font.data-(font.rowSize*6)*rows_down;

        memcpy(fb_pos, font_pos, buff_stride);
        
    }
    x_offset+=buff_stride;
    if(x_offset > buff_stride*width){
        y_offset+=buff_stride;
        x_offset = 0;
    }
    
}

void write(const char *buff, uint32_t count)
{
    for(int i = 0; i < count; i++){
        putChar(buff[i]);
    }
}

void printf(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    while (*fmt){
        if(*fmt == '%'){
            char specifier = *(fmt+1);
            switch (specifier)
            {
            case 'i':
                {
                    char buf[33];
                    int arg = va_arg(args, int);
                    itoa(arg, buf, 10);
                    write(buf, strlen(buf));
                }
                break;
            case 'd':
                {
                    char buf[33];
                    int arg = va_arg(args, int);
                    itoa(arg, buf, 10);
                    write(buf, strlen(buf));
                }
                break;
            case 'x':
                {
                    char buf[33];
                    int arg = va_arg(args, int);
                    itoa(arg, buf, 16);
                    write(buf, strlen(buf));
                }
                break;
            case 's':
                {
                    const char* arg = va_arg(args, const char*);
                    write(arg, strlen(arg));
                }
                break;
            case 'c':
                {
                    char arg = (char)va_arg(args, int);
                    putChar(arg);
                }
                break;

            default:
                break;
            }
            fmt += 2;
        }
        else if(*fmt == '\n'){
            y_offset+=24;
            x_offset = 0;
            fmt++;
        }
        else if(*fmt == '\r'){
            x_offset = 0;
            fmt++;
        }
        else{
            putChar(*fmt);
            fmt++;
        }

    }
    va_end(args);
}


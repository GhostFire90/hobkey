#include "terminal.h"
#include <stdint.h>
#include "psf.h"
#include "memops.h"
#include <stdarg.h>
#include "ramdisc.h"

#include "idt.h"
#include <limine.h>

#define CHARACTER_SIZE 6

static char* frameBuffer;
static uint32_t width,height;
static uint32_t x_offset, y_offset;
static psf_t font;
static uint64_t font_size;



void InitializeTerminal(struct limine_framebuffer* fb)
{
    GetPsf(&font, "resources/zap-vga.psf");
    //ReadBMP(FindFile("resources/font.bmp", &font_size), &font_b);
    x_offset = y_offset = 0;
    width = fb->width;
    height = fb->height;
    frameBuffer = (char*)fb->address;

    //setInterrupt(0x01, write, 0x8e);
}



void putChar(char c)
{
    
    for(uint32_t y = 0; y < font.height; y++){
        for(uint32_t x = 0; x < font.width; x++){
            int on = font.data[y*(font.width/8)+c*(font.height*(font.width/8))] & (1<<(font.width-x-1));
            uint32_t pix = 0xFFFFFFFF*on;
            memcpy(((char*)frameBuffer)+x_offset+(y_offset*(width*4))+(4*x)+(width*4*y), &pix, 4);
            
        }
    }
    x_offset+=font.width*4;
    // uint32_t buff_stride = (font.bit_depth/8)*CHARACTER_SIZE;
    // uint32_t frameBuffer_offset = c*buff_stride;

    // uint32_t rows_down = frameBuffer_offset/font.rowSize;
    // frameBuffer_offset -= font.rowSize*rows_down;
    

    // for(unsigned i = 0; i < CHARACTER_SIZE; i++){
    //     char* fb_pos = (y_offset*width)+x_offset+frameBuffer+(4*width*i);
    //     const char* font_pos = (frameBuffer_offset)+((font.height-i-1)*font.rowSize)+font.data-(font.rowSize*6)*rows_down;

    //     memcpy(fb_pos, font_pos, buff_stride);
        
    // }
    // x_offset+=buff_stride;
    // if(x_offset > buff_stride*width){
    //     y_offset+=buff_stride;
    //     x_offset = 0;
    // }
    
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
            int leading_zeros = 0; // mmm tasty
            if(*++fmt == '0'){
                fmt++;
                leading_zeros = 1;
            }
            int width = 0;
            while(*fmt >= '0' && *fmt <= '9'){
                width = width*10+(*fmt - '0');
                fmt++;
            }


            char specifier = *(fmt);
            char buf[33];
            switch (specifier)
            {
            case 'i':
                {
                    
                    int arg = va_arg(args, int);
                    //itoa(arg, buf, 10);
                    
                }
                break;
            case 'd':
                {
                   
                    int arg = va_arg(args, int);
                    //itoa(arg, buf, 10);
                }
                break;
            case 'x':
                {

                    int arg = va_arg(args, int);
                    //itoa(arg, buf, 16);
                    
                }
                break;
            case 'b':
                {
                    int arg = va_arg(args, int);
                    //itoa(arg, buf, 2);
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
                    buf[0] = arg;
                    buf[1] = 0;
                    //putChar(arg);
                }
                break;
            case 'p':
                {
                    char buff[65];
                    write("0x", 2);
                    unsigned long long arg = va_arg(args, unsigned long long);
                    //lltoa(arg, buff, 16);
                    write(buff, strlen(buff));
                    memset(buf, 0x0, 33);
                }
                break;
            case 'u':
            {
                uint32_t arg = va_arg(args, uint32_t);
                //uitoa(arg, buf, 10);
            }

            default:
                break;
            }
            int buf_len = strlen(buf);
            if(width && buf_len < width){
                width -= buf_len;
                while(width--){
                    if(leading_zeros)
                        putChar('0');
                    else
                        putChar(' ');
                }
                
            
            }

            write(buf, buf_len);
            fmt++;
        }
        else if(*fmt == '\n'){
            y_offset+=font.height;
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


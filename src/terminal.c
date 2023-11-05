#include "terminal.h"
#include "boot_param.h"
#include <stdint.h>
#include "bmp.h"
#include "helpers.h"
#include <stdarg.h>
#include "ramdisc.h"

#define CHARACTER_SIZE 6
#define BUFFER_SIZE 1024

static char* frameBuffer;
static uint32_t width,height;
static uint32_t offset;
static bmp_t font;
static uint64_t font_size;

static char text_buffer[BUFFER_SIZE];
static uint32_t text_buffer_pos;

static void StoreIntegerInBuffer(int32_t num){
    uint32_t i = 0;
    while(num){
        int32_t digit = num % 10;
        text_buffer[i+text_buffer_pos] = digit+'0';
        num = num/10;

        i++;
    }
    uint32_t old_pos = text_buffer_pos;
    text_buffer_pos+=i;
    for(uint32_t j = 0; j < i/2; j++){
        char c = text_buffer[old_pos+j];
        text_buffer[old_pos+j]=text_buffer[old_pos+(i-j)-1];
        text_buffer[old_pos+(i-j-1)] = c;        
    }
    text_buffer[text_buffer_pos] = 0;
}

void InitializeTerminal(struct BootParam* bp)
{
    ReadBMP(FindFile(bp->ramDisc, bp->ramDiscSize, "resources/font.bmp", &font_size), &font);
    offset = 0;
    width = bp->resX;
    height = bp->resY;
    frameBuffer = bp->frameBuffer;
    memset(text_buffer, 0, BUFFER_SIZE);
    text_buffer_pos = 0;
}



void putChar(char c)
{

    uint32_t buff_stride = (font.bit_depth/8)*CHARACTER_SIZE;
    uint32_t frameBuffer_offset = c*buff_stride;

    //memset(frameBuffer, 0xff, (width*4)*(height*4));
    //memcpy(frameBuffer, &frameBuffer_offset, sizeof(frameBuffer_offset));
   

    uint32_t rows_down = 0;
    while(frameBuffer_offset >= font.rowSize){
        frameBuffer_offset -= font.rowSize;
        rows_down++;
    }

    //memcpy(frameBuffer+sizeof(frameBuffer_offset), &rows_down, sizeof(rows_down));
    

    

    for(unsigned i = 0; i < CHARACTER_SIZE; i++){
        char* fb_pos = offset+frameBuffer+(4*width*i);
        const char* font_pos = (frameBuffer_offset)+((font.height-i-1)*font.rowSize)+font.data-(font.rowSize*6)*rows_down;

        memcpy(fb_pos, font_pos, buff_stride);
        
    }
    offset+=buff_stride;


    //memcpy(frameBuffer, font.data, CHARACTER_SIZE*4);
    
    
    
    // for(unsigned i = 0; i < CHARACTER_SIZE; i++){
    //     memcpy(&frameBuffer[offset+(i*(height*4))], &font.data[sub_sprite+(i*font.rowSize)], CHARACTER_SIZE*4);
    // }
}

void printf(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    while (*fmt)
    {
        if(*fmt == '%'){
            char specifier = *(fmt+1);
            switch (specifier){
                case 'd':
                    {
                        int32_t arg = va_arg(args, int32_t);
                        StoreIntegerInBuffer(arg);
                        
                        break;
                    }
                case 'i':{
                    int32_t arg = va_arg(args, int32_t);
                    StoreIntegerInBuffer(arg);
                    
                    break;
                }
                case 'c':
                    {
                        char arg = (char)va_arg(args, int);
                        text_buffer[text_buffer_pos] = arg;
                        text_buffer_pos++;
                        break;
                    }
                case 's':
                    {
                        const char* arg = va_arg(args, const char*);
                        strcpy(text_buffer+text_buffer_pos, arg);
                        text_buffer_pos+=strlen(arg);
                    }
            }
            fmt+=2;
        }
        else{
            text_buffer[text_buffer_pos] = *fmt;
            text_buffer_pos++;
            fmt++;
        }
        
    }
    text_buffer[text_buffer_pos] = 0;
    va_end(args);
}

uint32_t flush(void)
{
    uint32_t count = text_buffer_pos;
    for(uint32_t i = 0; i < text_buffer_pos; i++){
        putChar(text_buffer[i]);
    }
    text_buffer_pos = 0;
    return count;
}

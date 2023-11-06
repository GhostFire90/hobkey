#include "terminal.h"
#include "boot_param.h"
#include <stdint.h>
#include "bmp.h"
#include "string.h"
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

    uint32_t rows_down = frameBuffer_offset/font.rowSize;
    frameBuffer_offset -= font.rowSize*rows_down;
    

    for(unsigned i = 0; i < CHARACTER_SIZE; i++){
        char* fb_pos = offset+frameBuffer+(4*width*i);
        const char* font_pos = (frameBuffer_offset)+((font.height-i-1)*font.rowSize)+font.data-(font.rowSize*6)*rows_down;

        memcpy(fb_pos, font_pos, buff_stride);
        
    }
    offset+=buff_stride;
}

void write(const char *buff, uint32_t count)
{
    for(int i = 0; i < count; i++){
        putChar(buff[i]);
    }
}

void screenWipe()
{
    for(uint32_t i = 0; i < (width*4)*(height*4); i++){
        frameBuffer[i] = 0xff;
    }
}

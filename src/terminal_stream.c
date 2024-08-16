#include "terminal_stream.h"
#include "framebuffer_stream.h"
#include "psf.h"
#include "liballoc.h"

#define WHITE 0xFFFFFFFF

#define BITMAP_TO_ARRAY(bitmap, color) { \
    ((bitmap >> 7) & 1) * color, \
    ((bitmap >> 6) & 1) * color, \
    ((bitmap >> 5) & 1) * color, \
    ((bitmap >> 4) & 1) * color, \
    ((bitmap >> 3) & 1) * color, \
    ((bitmap >> 2) & 1) * color, \
    ((bitmap >> 1) & 1) * color, \
    ((bitmap >> 0) & 1) * color  \
}

typedef struct term_stream_s{
    char* buffer;
    stream_t* fb;
    psf_t font;
    unsigned y;
} terminal_t;

void term_putc(terminal_t* term, char byte){
    unsigned long pitch = framebuffer_get_pitch(term->fb);
    switch(byte){
        case '\n':
            term->y++;
            stream_seek(term->fb, term->y*pitch*term->font.height, STREAM_SEEK_START);
            return;
    }

    for(uint32_t y = 0; y < term->font.height; y++){
        char line = *(
              term->font.data
            + y
            + byte*term->font.height
            );
        uint32_t pixels[8] = BITMAP_TO_ARRAY(line, WHITE);
        stream_write(term->fb, (const char*)pixels, sizeof(uint32_t)*8);
        stream_seek(term->fb, pitch-(sizeof(uint32_t)*8), STREAM_SEEK_CUR);
    }
    stream_seek(term->fb, -((int64_t)term->font.height)*pitch + sizeof(uint32_t)*8, STREAM_SEEK_CUR);
    
}

long term_write(stream_t* stream, const char* bytes, unsigned long length){
    terminal_t* term = stream->functionality;
    unsigned short count = get_buffer_count(stream);
    unsigned long pitch = framebuffer_get_pitch(term->fb);

    for(uint64_t i = 0; i < length; i++){
        
        term->buffer[count] = bytes[i];
        count++;
        stream->flags &= ~BUFFER_COUNT_MASK;
        stream->flags |= count<<21;
        if(bytes[i] == '\n'){
            stream_flush(stream);
            count = 0;
        }
        
    }
    return length;

}
int term_flush(stream_t* stream){
    terminal_t* term = stream->functionality;
    unsigned short count = get_buffer_count(stream);

    for(uint16_t i = 0; i < count; i++){
        term_putc(term, term->buffer[i]);
    }
    return 0;
}

stream_t *create_terminal_stream(stream_t *fb_stream)
{

    stream_t* ret = kmalloc(sizeof(stream_t));
    terminal_t* term = kmalloc(sizeof(terminal_t));

    ret->flags = STREAM_F_SEEKABLE | STREAM_F_WRITEABLE | ((unsigned long)0x1000<<4);
    ret->flush = term_flush;
    ret->write = term_write;
    ret->functionality = (void*)term;
    
    term->buffer = kmalloc(4096);
    term->fb = fb_stream;

    GetPsf(&term->font, "resources/zap-vga.psf");
    
    return ret;
}

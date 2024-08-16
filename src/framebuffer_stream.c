#include "framebuffer_stream.h"
#include <stdint.h>
#include "liballoc.h"
#include "memops.h"

typedef struct framebuffer_stream_s framebuffer_stream_t;
struct framebuffer_stream_s
{
    char* start;
    char* current;
    char* end;
    uint64_t width, height;
    uint64_t pitch; //bytes in row
    uint16_t bpp;// bits per pixel

};
    

long fb_write(stream_t* stream, const char* buffer, unsigned long length){
    framebuffer_stream_t* fb_stream = stream->functionality;
    memcpy(fb_stream->current, buffer, length);
    fb_stream->current+=length;
    return length;
}

int fb_seek(stream_t* stream, int64_t offset, char whence){
    framebuffer_stream_t* fb = stream->functionality;
    char* position = 0;
    switch (whence)
    {
    case STREAM_SEEK_CUR:
        position = fb->current;
        break;
    case STREAM_SEEK_END:
        position = fb->end;
        break;
    case STREAM_SEEK_START:
        position = fb->start;
        break;
    default:
        return -1;
    }
    if (position + offset > fb->end || position + offset < fb->start){
        return -1;
    }
    fb->current = position + offset;
    
    
    return 0;
}

stream_t *create_framebuffer_stream(struct limine_framebuffer *fb)
{
    framebuffer_stream_t* fb_stream = kmalloc(sizeof(framebuffer_stream_t));
    stream_t* ret = kmalloc(sizeof(stream_t));

    fb_stream->width = fb->width;
    fb_stream->height = fb->height;
    fb_stream->pitch = fb->pitch;
    fb_stream->bpp = fb->bpp;
    fb_stream->start = fb_stream->current = fb->address;
    fb_stream->end = fb_stream->start + (fb_stream->height*fb_stream->pitch);

    ret->flags = STREAM_F_WRITEABLE | STREAM_F_SEEKABLE;
    ret->functionality = (void*)fb_stream;
    ret->write = fb_write;
    ret->seek = fb_seek;


    return ret;
}

unsigned long framebuffer_get_width(stream_t *fb)
{
    framebuffer_stream_t* fb_stream = fb->functionality;
    return fb_stream->width;
}

unsigned long framebuffer_get_height(stream_t *fb)
{
    framebuffer_stream_t* fb_stream = fb->functionality;
    return fb_stream->height;
}

unsigned long framebuffer_get_pitch(stream_t *fb)
{
    framebuffer_stream_t* fb_stream = fb->functionality;
    return fb_stream->pitch;
}

unsigned short framebuffer_get_bpp(stream_t *fb)
{
    framebuffer_stream_t* fb_stream = fb->functionality;
    return fb_stream->bpp;
}

#ifndef FRAMEBUFFER_STREAM_H
#define FRAMEBUFFER_STREAM_H

#include "limine.h"
#include "stream.h"



stream_t* create_framebuffer_stream(struct limine_framebuffer* fb);

unsigned long framebuffer_get_width(stream_t* fb);
unsigned long framebuffer_get_height(stream_t* fb);
unsigned long framebuffer_get_pitch(stream_t* fb);
unsigned short framebuffer_get_bpp(stream_t* fb);




#endif
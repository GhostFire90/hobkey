#ifndef FRAMEBUFFER_STREAM_H
#define FRAMEBUFFER_STREAM_H

#include "limine.h"
#include "stream.h"

typedef struct framebuffer_stream_s framebuffer_stream_t;

stream_t* create_framebuffer_stream(struct limine_framebuffer* fb);



#endif
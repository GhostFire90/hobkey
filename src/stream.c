#include "stream.h"

long stream_write(stream_t *stream, const char *bytes, unsigned long length)
{
    if(stream->flags & STREAM_F_WRITEABLE){
        return stream->write(stream, bytes, length);
    }
    return -1;
}

long stream_read(stream_t *stream, char *buffer, unsigned long length)
{
    if(stream->flags & STREAM_F_READABLE){
        return stream->read(stream, buffer, length);
    }
    return -1;
}

int stream_flush(stream_t *stream)
{
    if(stream->flags & STREAM_F_BUFFERED){
        return stream->flush(stream);
    }
    return -1;
}

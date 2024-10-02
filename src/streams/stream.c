#include "stream.h"

long stream_write(stream_t *stream, const char *bytes, unsigned long length)
{
    if(stream->flags & STREAM_F_WRITEABLE){
        // buffered logic
        if(stream->flush){

            unsigned short buffer_len = get_buffer_length(stream);
            unsigned short buffer_count = get_buffer_count(stream);

            unsigned short bytes_left = buffer_len-buffer_count;

            if(bytes_left < length){
                // have to write and flush!
                unsigned short left_over = length - bytes_left;
                stream->write(stream, bytes, bytes_left-1);
                stream->flush(stream);
                bytes+=bytes_left;
                
                stream->write(stream, bytes, left_over);

                //stream->flags &= ~BUFFER_COUNT_MASK;
                stream->flags |= left_over<<21;

                return length;
            }
            else{
                int ret = stream->write(stream, bytes, length);
                buffer_count += length;
                stream->flags &= ~BUFFER_COUNT_MASK;
                stream->flags |= buffer_count<<21;
                return ret;
            }

        }
        // non_buffered
        else{
            return stream->write(stream, bytes, length);
        }
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
    if(stream->flush){
        int ret = stream->flush(stream);
        stream->flags &= ~BUFFER_COUNT_MASK;
        return ret;
    }
    return -1;
}

int stream_seek(stream_t *stream, int64_t offset, char whence)
{
    if(stream->flags & STREAM_F_SEEKABLE){
        return stream->seek(stream, offset, whence);
    }
    return -1;
}

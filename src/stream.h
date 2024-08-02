#ifndef STREAM_H
#define STREAM_H

#define get_buffer_length(a) ((unsigned short)(a->flags>>4))
#define get_buffer_count(a) ((unsigned short)a->flags>>21) 
#define STREAM_F_BUFFERED  (1<<2)
#define STREAM_F_WRITEABLE (1<<1)
#define STREAM_F_READABLE  1

typedef struct stream_s stream_t;

// WritePred takes a stream to write to, a buffer to write, and the length of the buffer
typedef long (*WritePred)(stream_t*, const char*, unsigned long);
// ReadPred takes a stream to read from, a buffer to store in, and a buffer length;
typedef long (*ReadPred)(stream_t*, char*, unsigned long);

typedef int (*FlushPred)(stream_t*);

/*
Do not directly call or interface with this struct, only visible for other stream types to implement it

write: pointer function called by a write, ignored if the write flag is 0, returns -1 if fails otherwise length of write
read:  pointer function called by a read, ignored if the read flag is 0, returns -1 if fails otherwise length of write 


Flags:
+--------------+---------------+----------+----------+-------+------+
|    36-21     |     20-4      |    3     |    2     |   1   |  0   |
+--------------+---------------+----------+----------+-------+------+
| buffer_count | buffer_length | blocking | buffered | write | read |
+--------------+---------------+----------+----------+-------+------+
*/
struct stream_s{
    WritePred write;
    ReadPred read;
    FlushPred flush;
    unsigned long flags;
    void* functionality;    
};

long stream_write(stream_t* stream, const char* bytes, unsigned long length);
long stream_read(stream_t* stream, char* buffer, unsigned long length);
int stream_flush(stream_t* stream);

#endif
#ifndef RAMDISC_H
#define RAMDISC_H

#include <stddef.h>
#include <stdint.h>

typedef enum FileType{
    Normal = 0,
    Hard = 1,
    Sym = 2,
    CharDev = 3,
    BlockDev = 4,
    Dir = 5,
    FIFOPipe = 6
} FileType;

struct FileInfo{

    char name[256];
    unsigned long size;
    FileType type;
    unsigned char* data;
    
};

unsigned char *FindFile(char* ramdiscLocation, unsigned long ramdiscSize, const char *path, unsigned long* fileSize);
int GetFileInfo(unsigned char* address, struct FileInfo *ret);



#endif

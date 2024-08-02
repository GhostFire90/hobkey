#include "ramdisc.h"
#include "memops.h"
#define USTAR_SECTOR_SIZE 512
#define USTAR_MAG_OFFSET 257
#define USTAR_MAG_IDENT "ustar"
#define USTAR_NAMEPREFIX_OFFSET 345
#define USTAR_NAMEPREFIX_LENGTH 155
#define USTAR_NAME_OFFSET 0
#define USTAR_NAME_LENGTH 100
#define USTAR_TYPE_OFFSET 156
#define USTAR_SIZE_OFFSET 124
#define USTAR_SIZE_LENGTH 12

char *ramdiscLocation;
unsigned long ramdiscSize;

static unsigned long FileSizeFromOctal(const unsigned char* input, unsigned long len){
    unsigned long ret = 0;
    unsigned long startIndex = 0;
    while (startIndex < len && input[startIndex] == '0') {
        startIndex++;
    }

    if (startIndex == len) {
        // The string is all zeros
        return 0;
    }
    for (unsigned long i = startIndex; i < len; i++) {
        ret = ret * 8 + (input[i] - '0');
    }
    return ret;
}

int GetFileInfo(unsigned char* address, struct FileInfo *ret){
    
    int n = memcmp(address+USTAR_MAG_OFFSET, USTAR_MAG_IDENT, 6);
    if(n == 0){
        unsigned long nameOffset = 0;
        nameOffset += strlen(address+USTAR_NAMEPREFIX_OFFSET);
        strcpy(ret->name, address+USTAR_NAMEPREFIX_OFFSET);
        strcpy(ret->name+nameOffset, address+USTAR_NAME_OFFSET);
        nameOffset += strlen(address+USTAR_NAME_OFFSET);
        ret->type = address[USTAR_TYPE_OFFSET] ? address[USTAR_TYPE_OFFSET]-'0' : 0;
        ret->data = address+USTAR_SECTOR_SIZE;
        ret->name[nameOffset] = 0;
        if(ret->type != Dir)
            ret->size = FileSizeFromOctal(address+USTAR_SIZE_OFFSET, USTAR_SIZE_LENGTH-1);
        else
            ret->size = 0;
        return 1;
    }
    return 0;
}

void InitializeRamdisc(char *ramdiscLocation_, unsigned long ramdiscSize_)
{
    ramdiscLocation = ramdiscLocation_;
    ramdiscSize = ramdiscSize_;
}

unsigned char *FindFile(const char *path, unsigned long *fileSize)
{
    unsigned sectorCount = 0;
    struct FileInfo f;
    int res = GetFileInfo(ramdiscLocation, &f);
    while(res && sectorCount*USTAR_SECTOR_SIZE < ramdiscSize){
        if(strcmp(f.name, path) == 0){
            *fileSize = f.size;
            return f.data;
        }
        else{
            f.data = NULL;
        }
        unsigned sectors = (f.size + USTAR_SECTOR_SIZE - 1) / USTAR_SECTOR_SIZE;
        sectorCount += sectors+1;
        
        res = GetFileInfo(ramdiscLocation+USTAR_SECTOR_SIZE*sectorCount, &f);
        
    }
    
    return NULL;
}


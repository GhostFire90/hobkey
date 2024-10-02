#define PSF1_MAG "\x36\x04"
#define PSF2_MAG "\x72\xb5\x4a\x86"

#include "psf.h"
#include "ramdisc.h"
#include "memops.h"

#define ROUND_UP_TO_POWER_OF_2(num) \
    do {                             \
        --(num);                     \
        (num) |= (num) >> 1;         \
        (num) |= (num) >> 2;         \
        (num) |= (num) >> 4;         \
        (num) |= (num) >> 8;         \
        (num) |= (num) >> 16;        \
        ++(num);                     \
    } while (0)

#define IS_POWER_OF_2(num) ((num) && !((num) & ((num) - 1)))



void GetPsf(psf_t *psf, const char *path)
{
    uint64_t size;
    unsigned char* file_data = FindFile(path, &size);
    if(memcmp(file_data, PSF1_MAG, 2) == 0){
        //is psf1
        struct psf1_header* header = (struct psf1_header*)(file_data);
        psf->width = 8;
        psf->height = (uint32_t)header->dat[1];
        psf->byte_count = psf->width;
        psf->mode = (uint32_t)header->dat[0];
        psf->data = file_data+4;
        psf->data_len = size-4;
    }
    else if(memcmp(file_data, PSF2_MAG, 4) == 0){
        //is psf2
    }
    else{
        //idfk what this is
    }
    return;
}

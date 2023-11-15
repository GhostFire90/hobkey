#define PSF1_MAG "\x36\x04"
#define PSF2_MAG "\x72\xb5\x4a\x86"

#include "psf.h"
#include "ramdisc.h"
#include "string.h"

void GetPsf(psf_t *psf, const char *path)
{
    uint64_t size;
    unsigned char* file_data = FindFile(path, &size);
    if(memcmp(file_data, PSF1_MAG, 2) == 0){
        //is psf1
        struct psf1_header* header = (struct psf1_header*)(file_data);
        psf->width = 8;
        psf->height = (uint32_t)header->dat[1];
        psf->byte_count = psf->width*psf->height;
        psf->mode = (uint32_t)header->dat[0];
        psf->data = file_data+sizeof(struct psf1_header);
        psf->data_len = size-sizeof(struct psf1_header);
    }
    else if(memcmp(file_data, PSF2_MAG, 4) == 0){
        //is psf2
    }
    else{
        //idfk what this is
    }
    return;
}

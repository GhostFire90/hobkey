#ifndef PSF_H
#define PSF_H

#include <stdint.h>

struct psf1_header{
    uint16_t mag;
    unsigned char dat[2];
};
struct psf2_header{
    uint32_t mag;
    uint32_t version;
    uint32_t header_size;
    uint32_t flags;
    uint32_t length;
    uint32_t glyph_byte_count;
    uint32_t height;
    uint32_t width;    
};

typedef struct psf_info{
    uint32_t width, height;
    uint32_t byte_count;
    uint32_t mode;
    char* data;
    uint64_t data_len;

} psf_t;

void GetPsf(psf_t* psf, const char* path);

#endif

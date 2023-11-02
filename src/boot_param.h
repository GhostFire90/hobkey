#ifndef BOOT_PARAM_H
#define BOOT_PARAM_H

#include <stddef.h>
#include <stdint.h>

struct BootParam {
    unsigned char* frameBuffer;
    uint32_t resX, resY;
    uint32_t pitch;

    unsigned char* ramDisc;
    uint64_t ramDiscSize;
};

#endif
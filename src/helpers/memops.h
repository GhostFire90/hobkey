#ifndef HELPERS_H
#define HELPERS_H

#include <stdint.h>

extern void memcpy(void* dest, const void* src, unsigned long size);
extern int32_t memcmp(const void* first, const void* second, unsigned long size);
extern void memset(void* mem, char val, unsigned long size);
extern void strcpy(char* dest, const char* src);
extern int32_t strcmp(const char* first, const char* second);
extern unsigned strlen(const char* str);

#endif

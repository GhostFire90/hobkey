#ifndef HELPERS_H
#define HELPERS_H

#include <stdint.h>

void memcpy(void* dest, const void* src, unsigned long size);
int32_t memcmp(const void* first, const void* second, unsigned long size);
void memset(void* mem, char val, unsigned long size);
void strcpy(char* dest, const char* src);
int32_t strcmp(const char* first, const char* second);
unsigned strlen(const char* str);

uint32_t count_digits(int32_t num);

#endif

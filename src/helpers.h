#ifndef HELPERS_H
#define HELPERS_H

void memcpy(void* dest, const void* src, unsigned long size);
int memcmp(const void* first, const void* second, unsigned long size);
void memset(void* mem, char val, unsigned long size);
void strcpy(char* dest, const char* src);
int strcmp(const char* first, const char* second);
unsigned strlen(const char* str);

#endif

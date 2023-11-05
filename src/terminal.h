#ifndef TERMINAL_H
#define TERMINAL_H

#include <stdint.h>

struct BootParam;

void InitializeTerminal(struct BootParam* bp);

void putChar(char c);
void printf(const char* fmt, ...);
uint32_t flush(void);

#endif
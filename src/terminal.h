#ifndef TERMINAL_H
#define TERMINAL_H

#include <stdint.h>

struct BootParam;

void InitializeTerminal(struct BootParam* bp);

void putChar(char c);
void write(const char* buff, uint32_t count);

void printf(const char* fmt, ...);
//void screenWipe();
#endif
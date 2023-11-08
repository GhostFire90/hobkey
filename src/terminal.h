#ifndef TERMINAL_H
#define TERMINAL_H

#include <stdint.h>
#include <limine.h>


void InitializeTerminal(struct limine_file* ramdisc, struct limine_framebuffer* fb);

void putChar(char c);
void write(const char* buff, uint32_t count);

void printf(const char* fmt, ...);
//void screenWipe();
#endif
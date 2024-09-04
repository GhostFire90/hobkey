#ifndef IDT_H
#define IDT_H
#include <stdint.h>

#define GATE_INTERRUPT 0xE
#define GATE_TRAP 0xF

void setInterrupt(uint8_t int_num, void* callback, uint8_t type);
void refresh_idt(void);

#endif
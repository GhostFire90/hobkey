#ifndef IDT_H
#define IDT_H
#include <stdint.h>

void setInterrupt(uint8_t int_num, void* callback, uint8_t type);
void refresh_idt(void);

#endif
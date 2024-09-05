#include <stdint.h>
#include <stddef.h>
#include "idt.h"

struct InterruptDescriptor64 {
   uint16_t offset_1;        // offset bits 0..15
   uint16_t selector;        // a code segment selector in GDT or LDT
   uint8_t  ist;             // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
   uint8_t  type_attributes; // gate type, dpl, and p fields
   uint16_t offset_2;        // offset bits 16..31
   uint32_t offset_3;        // offset bits 32..63
   uint32_t zero;            // reserved
};

void empty_int(void);



struct InterruptDescriptor64* idtr;


void createIDTEntry(struct InterruptDescriptor64* entry, uint64_t callback, uint8_t type){
    uint64_t s = sizeof(*entry);
    entry->offset_1 = 0xFFFF & callback;
    entry->offset_2 = 0xFFFF & (callback >> 16);
    entry->offset_3 = 0xFFFFFFFF & (callback >> 32);
    entry->type_attributes = type;
    entry->selector = 0x08;
    entry->ist = 0;

}

struct InterruptDescriptor64* GetIDTR(struct InterruptDescriptor64* idtr_){
    idtr = idtr_;
    for(uint32_t i = 0; i < 256; i++){
        createIDTEntry(&idtr[i], (uint64_t)empty_int, 0x80 | GATE_INTERRUPT);
    }
    
    return idtr;
}

void setInterrupt(uint8_t int_num, void* callback, uint8_t type){
    createIDTEntry(&idtr[int_num], (uint64_t)callback, 0x80 | GATE_INTERRUPT);
    refresh_idt();
}
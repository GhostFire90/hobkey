#include "apic.h"
#include "system_tables.h"
#include "idt.h"
#include "virtual_memory_management.h"
#include "stream.h"
#include <stdint.h>

#define PIC1 0x20
#define PIC2 0xA0



#define SIVR_OFFSET 0x0f0
#define EOI_OFFSET  0x0b0

// Task Priority regs 
#define TPR_OFFSET  0x080
#define PPR_OFFSET  0x0A0

// Timer regs
#define ICNT_OFFSET 0x380 // Initial count, uint32_t
#define CCNT_OFFSET 0x390 // Current count, uint32_t
#define DCNF_OFFSET 0x3e0 // Divide config, only access bits 0,1,3
#define TIVR_OFFSET 0x320 // LVT entry for timer, set this to the interrupt you want, uint32_t

extern void empty_int();
extern void apic_test();


struct MADT{
    struct ACPISDTHeader header;
    uint32_t local_apic;
    uint32_t flags;
};

static volatile char* apic_address;
extern stream_t* get_terminal();

static inline void outb(uint16_t port, uint8_t val)
{
    __asm__ volatile ( "outb %b0, %w1" : : "a"(val), "Nd"(port) : "memory");
    /* There's an outb %al, $imm8 encoding, for compile-time constant port numbers that fit in 8b. (N constraint).
     * Wider immediate constants would be truncated at assemble-time (e.g. "i" constraint).
     * The  outb  %al, %dx  encoding is the only option for all other cases.
     * %1 expands to %dx because  port  is a uint16_t.  %w1 could be used if we had the port number a wider C type */
}

void apic_initialize(void){

    outb(PIC1+1, 0xff);
    outb(PIC2+1, 0xff);

    struct MADT* madt = find_table(MADT_SIGNATURE);
    apic_address = extend_kernel_map((void*)(uint64_t)madt->local_apic);
    setInterrupt(0xFF, empty_int, GATE_INTERRUPT);

    uint32_t* sivr = (uint32_t*)(apic_address+SIVR_OFFSET);
    *sivr = 0xff | (1<<8);


    
    uint32_t* apic_tpr = (uint32_t*)(apic_address+TPR_OFFSET);
    *apic_tpr = 0x30;

    setInterrupt(0xFE, apic_test, GATE_INTERRUPT);
    uint32_t* timer_reg = (uint32_t*)(apic_address+TIVR_OFFSET);
    *timer_reg = 0xFE | (1<<17);

    uint32_t* divisor = (uint32_t*)(apic_address+DCNF_OFFSET);
    *divisor = 10;

    uint32_t* initial_count = (uint32_t*)(apic_address+ICNT_OFFSET);
    *initial_count = 0xFFF;

    
}
void apic_test_C(void){
//  stream_write(get_terminal(), "h\n", 2);
    volatile uint32_t* EOI = (uint32_t*)(apic_address+EOI_OFFSET);
    *EOI=0;
    return;
}


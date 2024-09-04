#include "apic.h"
#include "system_tables.h"
#include "idt.h"
#include "virtual_memory_management.h"
#include <stdint.h>

#define SIVR_OFFSET 0xf0

extern void empty_int();

struct MADT{
    struct ACPISDTHeader header;
    uint32_t local_apic;
    uint32_t flags;
};

static char* apic_address;

void apic_initialize(void){
    struct MADT* madt = find_table(MADT_SIGNATURE);
    apic_address = extend_kernel_map((void*)(uint64_t)madt->local_apic);
    setInterrupt(0xFF, empty_int, GATE_INTERRUPT);
    apic_address[SIVR_OFFSET/8] = 0xff;
    apic_address[SIVR_OFFSET/8+1]=1;
}
void apic_test_C(void){
    return;
}


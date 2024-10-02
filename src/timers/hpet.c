#include "hpet.h"
#include "hpet.h"
#include <helpers/system_tables.h>
#include <stdint.h>

struct hpet_table{
    struct ACPISDTHeader header;
    uint32_t TimerBlockID;
    uint8_t AddressSpace, RegBitWidth, RegBitOffset, Reserved;
    uint64_t Address;
    uint8_t HPET_Number;
    uint16_t MainCounterMin;
    uint8_t PageProtection;
};

void hpet_initialize()
{
    struct hpet_table* table = find_table(HPET_SIGNATURE);
}
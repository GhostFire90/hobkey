#ifndef SYSTAB_H
#define SYSTAB_H
#include <stdint.h>

#define MADT_SIGNATURE "APIC"


struct ACPISDTHeader {
  char Signature[4];
  uint32_t Length;
  uint8_t Revision;
  uint8_t Checksum;
  char OEMID[6];
  char OEMTableID[8];
  uint32_t OEMRevision;
  uint32_t CreatorID;
  uint32_t CreatorRevision;
};

void initialize_tables(void* dp_address);
void* find_table(const char* signature);

#endif
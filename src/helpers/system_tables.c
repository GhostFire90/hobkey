#include <helpers/system_tables.h>
#include <stdint.h>
#include <helpers/limine_requests.h>
#include <helpers/memops.h>
#include <memory/paging.h>
#include <memory/virtual_memory_management.h>

struct XSDT{
    struct ACPISDTHeader header;
    uint64_t first_entry;
};

struct XSDP_t {
 char Signature[8];
 uint8_t Checksum;
 char OEMID[6];
 uint8_t Revision;
 uint32_t RsdtAddress;      // deprecated since version 2.0
 
 uint32_t Length;
 uint64_t XsdtAddress;
 uint8_t ExtendedChecksum;
 uint8_t reserved[3];
} __attribute__ ((packed));

static struct XSDT* xsdt;

int checksum_validate(struct ACPISDTHeader *header)
{
    unsigned char sum = 0;
 
    for (int i = 0; i < header->Length; i++)
    {
        sum += ((char *) header)[i];
    }
 
    return sum == 0;
}

void initialize_tables(void* dp_address)
{
    struct XSDP_t* dp = dp_address;
    uint64_t addr = dp->XsdtAddress;
    uint64_t remainder = addr % PAGE_SIZE;
    addr = addr-remainder;
    char* page = extend_kernel_map((void*)addr);
    xsdt = (struct XSDT*) (page+remainder);
    //xsdt->first_entry >>=32; // 4 byte aligned lol
    

}

void *find_table(const char *signature)
{
    uint64_t* entries = &xsdt->first_entry;
    int entry_count = (xsdt->header.Length-sizeof(xsdt->header))/8;
    
    for(int i = 0; i < entry_count; i++){
        struct ACPISDTHeader* entry = map_temp_nearest((void*)(entries[i]>>32));
        if(memcmp(entry->Signature, signature, 4) == 0){
            if(!checksum_validate(entry)){
                unmap_temp();
                return 0;
            }
            unmap_temp();
            return extend_kernel_map((void*)(entries[i]>>32));
        }        
    }

    return 0;
}

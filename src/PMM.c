#include "PMM.h"
#include <stdint.h>
#include <limine.h>
#include "terminal.h"
#include "limine_requests.h"

#define PAGE_SIZE 4096


typedef struct _free_page{
    void* previous;
} freepage_t;



static freepage_t* top = 0;
static int free_pages = 0;

void* list_from_block(const struct limine_memmap_entry* entry, void* prev){
    //uint64_t page_count = entry.length/PAGE_SIZE;

    uint64_t offset = limine_hhdm()->offset;
    uint64_t base = entry->base;
    uint64_t end = base + entry->length;
    for(uint64_t current = base; current < end; current += PAGE_SIZE){
        freepage_t* pg = (freepage_t*)(current+offset);
        pg->previous = prev;
        prev = (void*)current;
        free_pages++;
        
    }
    return prev;
}

int page_count(void){
    return free_pages;
}
void build_list()
{
    const struct limine_memmap_response* memmap = limine_memmap();
    void* current = 0;
    for(uint64_t i = 0; i < memmap->entry_count; i++){
        struct limine_memmap_entry* entry = memmap->entries[i];
        if(entry->type == LIMINE_MEMMAP_USABLE){
            current = list_from_block(entry, current);

        }
    }
    top = (freepage_t*)current;
    printf("Prepared %d pages\n", free_pages);
}

void *get_page()
{
    return (void*)top;
}

void allocate_page(void* vaddr){
    
    freepage_t* fp = (freepage_t*)vaddr;
    top = fp->previous;
    free_pages--;
}

void free_page(void* page)
{
    freepage_t* fp = (freepage_t*)page;
    //fp->previous
}
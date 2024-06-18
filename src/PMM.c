#include "PMM.h"
#include <stdint.h>
#include <limine.h>
#include "terminal.h"
#include "limine_requests.h"
#include "paging.h"

#define PAGE_SIZE 4096


typedef struct _free_page{
    struct _free_page* previous;
} freepage_t;



static freepage_t* top = 0;
static int free_pages = 0;

freepage_t* list_from_block(const struct limine_memmap_entry* entry, freepage_t* prev){
    //uint64_t page_count = entry.length/PAGE_SIZE;

    uint64_t offset = limine_hhdm()->offset;
    uint64_t base = entry->base;
    uint64_t end = base + entry->length;
    for(uint64_t current = base; current < end; current += PAGE_SIZE){
        freepage_t* pg = (freepage_t*)(current+offset);
        pg->previous = prev;
        prev = (freepage_t*)current;
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
    freepage_t* current = 0;
    for(uint64_t i = 0; i < memmap->entry_count; i++){
        struct limine_memmap_entry* entry = memmap->entries[i];
        if(entry->type == LIMINE_MEMMAP_USABLE){
            current = list_from_block(entry, current);

        }
    }
    top = current;
    printf("Prepared %d pages\n", free_pages);
}

void *get_page()
{
    return (void*)top;
}

void allocate_page(void* vaddr){
    
    freepage_t* fp = (freepage_t*)vaddr;
    top = fp->previous;
    //fp->previous = 0;
    free_pages--;
}

// OF NOTE, THIS DOES NOT UNMAP IT, JUST PUSHES IT ONTO THE STACK, YOU HAVE BEEN WARNED 👿
void free_page(void* page)
{
    
    freepage_t* fp = (freepage_t*)page;
    if(CustomPagingEnabled()){
        uint64_t physical = get_pointer(*map_crawl((uint64_t)page, LAYER_PT));
        
        fp->previous = top;
        top = (freepage_t*)physical;
    }
    else{
        uint64_t offset = limine_hhdm()->offset;

        fp->previous = top;
        top = (freepage_t*)((char*)page - offset);
    }
    
    //fp->previous
}
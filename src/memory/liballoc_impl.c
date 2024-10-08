
#include "virtual_memory_management.h"
#include <spinlock.h>

static spinlock_t lock = 0; 


/** This function is supposed to lock the memory data structures. It
 * could be as simple as disabling interrupts or acquiring a spinlock.
 * It's up to you to decide. 
 *
 * \return 0 if the lock was acquired successfully. Anything else is
 * failure.
 */
int liballoc_lock(){
    acquire_lock(&lock);
    return 0;
}

/** This function unlocks what was previously locked by the liballoc_lock
 * function.  If it disabled interrupts, it enables interrupts. If it
 * had acquiried a spinlock, it releases the spinlock. etc.
 *
 * \return 0 if the lock was successfully released.
 */
int liballoc_unlock(){
    release_lock(&lock);
    return 0;
}

/** This is the hook into the local system which allocates pages. It
 * accepts an integer parameter which is the number of pages
 * required.  The page size was set up in the liballoc_init function.
 *
 * \return NULL if the pages were not allocated.
 * \return A pointer to the allocated memory.
 */
void* liballoc_alloc(int count){
    void* start = 0;
    for(int i = 0; i < count; i++){
        void* res = vm_allocate_page();
        if(res == 0){
            return 0;
        }
        if(start == 0){
            start = res;
        }
    }
    return start;
}

/** This frees previously allocated memory. The void* parameter passed
 * to the function is the exact same value returned from a previous
 * liballoc_alloc call.
 *
 * The integer value is the number of pages to free.
 *
 * \return 0 if the memory was successfully freed.
 */
int liballoc_free(void* start_address, int count){
    char* current = start_address;
    for(int i = 0; i < count; i++){
        vm_free_page(current);
        current += 0x1000;
    }
    
    return 0;
}
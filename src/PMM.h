#ifndef PMM_H
#define PMM_H

void build_list();
void* get_page();
void allocate_page(void* vaddr);
void free_page(void* page);


#endif

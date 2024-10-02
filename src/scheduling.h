#ifndef SCHEDULING_H
#define SCHEDULING_H

#include <stdint.h>

typedef struct process_s{
    uint64_t RIP;
    uint64_t RSP, RBP;
    uint64_t rax, rbx, rcx, rdx, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15;
} process_t;




#endif
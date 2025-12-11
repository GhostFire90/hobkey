
.bss
    .globl MAX_PHY_BIT
    .globl MAX_VRT_BIT
    .globl STACK_TOP

    STACK_BOTTOM:
    .space (1024*16), 0
    STACK_TOP: 

    MAX_PHY_BIT:  .space 1
    MAX_VRT_BIT:  .space 1
    
.text
    get_phybit:
        mov eax, 0x80000008
        cpuid

        mov [MAX_PHY_BIT], AL
        mov [MAX_VRT_BIT], AH
        ret



    .globl _start
    
    _start:
        pop rax
        lea rsp, STACK_TOP

        call setGdt
        call setup_idt
        call get_phybit
        call kmain
        lp:
        jmp lp
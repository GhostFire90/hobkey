
.bss
    .globl MAX_PHY_BIT
    .globl MAX_VRT_BIT
    MAX_PHY_BIT:
        .space 1
    MAX_VRT_BIT:
        .space 1
.text
    get_phybit:
        mov eax, 0x80000008
        cpuid
        mov ebx, eax
        and eax, 0xff
        and ebx, 0xff00
        shr ebx, 8
        mov (MAX_PHY_BIT), AL
        mov (MAX_VRT_BIT), BL
        ret



    .globl _start
    _start:
        call setGdt
        call kmain
        lp:
        jmp lp
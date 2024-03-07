section .bss
    global MAXPHYBIT
    MAXPHYBIT resb 1
    ;ALIGN 16
    ;stack_bottom:
    ;    TIMES 16384 db
    ;stack_top:

section .data
    

section .text
    extern kernel_main
    extern setup_idt
    extern setGdt
    global _start

    get_phybit:
        mov eax, 0x80000008
        cpuid
        and eax, 0xff
        mov [MAXPHYBIT], eax
        ret

    _start:

        ;mov rdx,CR0                            ; Start probe, get CR0
        ;and rdx, ~(1<<2)
        ;and rdx, ~(1<<3)
        ;
        ;mov CR0, rdx                            ; store control word
        ;FNINIT   

        ;mov ebx, esp
        ;mov esp, $stack_top  

        call setGdt
        call setup_idt
    
        call get_phybit
        call kernel_main
        
        

        ;mov esp, ebx
        
        hlt
        jmp 1b
        
        ;mov eax, 42
        ret
    param_conversion:
        ret
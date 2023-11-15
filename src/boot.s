section .bss
    ;ALIGN 16
    ;stack_bottom:
    ;    TIMES 16384 db
    ;stack_top:

section .text
    extern kernel_main
    extern setGdt
    global _start
    _start:

        ;mov rdx,CR0                            ; Start probe, get CR0
        ;and rdx, ~(1<<2)
        ;and rdx, ~(1<<3)
        ;
        ;mov CR0, rdx                            ; store control word
        ;FNINIT   

        ;mov ebx, esp
        ;mov esp, $stack_top  
        cli
        call setGdt
        call kernel_main
        
        ;mov esp, ebx
        
        hlt
        jmp 1b
        
        ;mov eax, 42
        ret
    param_conversion:
        ret
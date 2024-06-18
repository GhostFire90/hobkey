section .bss
    global MAXPHYBIT
    global MAXVRTBIT
    global STACK_ADDRESS
    ;global stack_bottom
    ;align 16
    ;stack_bottom:
    ;    resb 65536
    ;stack_top:
    MAXPHYBIT resb 1
    MAXVRTBIT resb 1
    STACK_ADDRESS resb 8
    

section .data
    

section .text
    extern kernel_main
    extern setup_idt
    extern setGdt
    global _start

    get_phybit:
        mov eax, 0x80000008
        cpuid
        mov ebx, eax
        and eax, 0xff
        and ebx, 0xff00
        shr ebx, 8
        mov byte [MAXPHYBIT], AL
        mov byte [MAXVRTBIT], BL
        ret

    _start:
        pop rax
        mov [STACK_ADDRESS], rsp

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
section .text
    global apic_test
    extern apic_test_C

    
%macro PUSHA 0
    push rax
    push rcx
    push rdx
    push r8
    push r9
    push r10
    push r11
%endmacro

%macro POPA 0
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdx
    pop rcx
    pop rax
%endmacro

    apic_test:
        PUSHA
    
        cld
        call apic_test_C
        
        POPA
        
        iretq


    
section .data
    idt_begin:
        times 512 dq 0
    idt_end:


    idtr:
        dw idt_end-idt_begin-1
        dq idt_begin

section .text
    
    global setup_idt
    global empty_int
    global refresh_idt
    extern GetIDTR
    
    
    setup_idt:
        lea rdi, idt_begin
        call GetIDTR
        call refresh_idt

        ret
    refresh_idt:
        cli
        lidt [idtr]
        sti
        ret

    empty_int:
        iret


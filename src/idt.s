.data
    idt_begin:
        .space 4096, 0
    idt_end:
    idtr:
        .word idt_end-idt_begin-1
        .quad idt_begin
.text

    .globl setup_idt
    .globl empty_int
    .globl refresh_idt
    .globl get_idtr
    .extern IDTR_init

    setup_idt:
        lea rdi, idt_begin
        call IDTR_init
        call refresh_idt
        ret

    refresh_idt:
        cli
        lidt [idtr]
        sti
        ret
    get_idtr:
        lea rax, idt_begin
        ret

    empty_int:
        iretq

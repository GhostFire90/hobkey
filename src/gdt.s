.data
    .align 8
    gdt_desc:
        .quad 0
        .quad 0x00af9b000000ffff
        .quad 0x00af93000000ffff
        .quad 0x00affb000000ffff
        .quad 0x00aff3000000ffff
    gdt_desc_end:

    gdtr:
        .word (gdt_desc_end)-(gdt_desc) - 1
        .quad (gdt_desc)
.text
    .globl setGdt
    setGdt:
        lgdt (gdtr)
        call reloadSegments
    reloadSegments:
        push 0x08
        lea rax, (rip+.reload_cs)
        push rax
        retfq
    .reload_cs:
        mov ax, 0x10
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov ss, ax
        ret
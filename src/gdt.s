   
section .data
    align 8


    gdt_desc:
        dq 0
        dq 0x00af9b000000ffff
        dq 0x00af93000000ffff
        dq 0x00affb000000ffff
        dq 0x00aff3000000ffff
    gdt_desc_end:

    gdtr:
        dw gdt_desc_end-gdt_desc - 1
        dq gdt_desc


section .text
    global setGdt
    setGdt:
        ;lea rdi, [gdt_desc]
        ;call create_gdt
    
        lgdt [gdtr]
        call reloadSegments
    reloadSegments:
        push 0x08
        lea rax, [rel .reload_cs]
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

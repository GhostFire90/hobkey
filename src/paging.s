
section .text

    global set_cr3
    global create_mask
    create_mask:
        mov rax, 1
        dec rdi
        mov rsi, 1
        .cm_loop:
            shl rsi, 1
            or rax, rsi
            dec rdi
            jnz .cm_loop
        ret
    set_cr3:
        mov cr3, rdi
        nop
        ret

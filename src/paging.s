
section .text

    global create_mask
    create_mask:
        mov rax, 1
        mov rcx, rdi
        shl rax, cl
        dec rax
        ret


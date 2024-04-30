
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
        
        mov rbx, cr0
        and ebx, ~(1<<31)
        mov cr0, rbx

        shl rdi, 12
        mov cr3, rdi

        or ebx, (1<<31)
        mov cr0, rbx

        mov ax, 0x10
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        
        ret 

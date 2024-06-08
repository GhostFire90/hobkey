
section .text

    global create_mask
    ; (1<<n)-1
    create_mask:
        mov rax, 1      ; initialize mask
        mov rcx, rdi    ; move rdi into rcx, since cl is the lower 8 bits of rcx
        shl rax, cl     ; shift rax left by cl (one of the only regs allowed for rhs of this inst)
        dec rax         ; subtract one
        ret             ; return mask


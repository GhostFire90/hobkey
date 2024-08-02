
section .text
    global count_digits

    count_digits:
        mov eax, edi
        xor rdi, rdi
        _count_digits_loop:
            mov ecx, 10
            div ecx
            inc edi
            test eax, eax
            jnz _count_digits_loop
        mov eax, edi
        ret

            
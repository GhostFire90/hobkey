section .text
    global memcpy
    global memset
    global memcmp
    global strcpy
    global strcmp
    global strlen
    memcpy:
        mov rcx, rdx
        rep movsb
        ret
    memset:
        _msetloop:
            mov [rdi+rdx-1], sil
            dec rdx
            jnz _msetloop
        ret
    strcmp:
        xor eax, eax
        _strcmploop:
            mov cl, byte [rdi]
            mov ch, byte [rsi]
            test ch, ch
            jz _strcmpend
            sub cl, ch
            movzx ecx, cl
            add eax, ecx
            inc rdi
            inc rsi
            jmp _strcmploop

        _strcmpend:
            ret
    strlen:
        xor eax, eax
        _strlenloop:
            mov cl, byte [rdi]
            test cl, cl
            jz _strlenend
            inc eax
            inc rdi
            jmp _strlenloop
        _strlenend:
            ret
    strcpy:
        _strcpyloop:
            mov cl, byte[rsi]
            mov [rdi], cl
            test cl, cl
            jz _strcpyend
            inc rdi
            inc rsi
            jmp _strcpyloop
        _strcpyend:
            ret
    memcmp:
        xor eax, eax
        dec rdx
        _memcmploop:
            mov cl, [rdi+rdx]
            mov ch, [rsi+rdx]
            sub cl, ch
            movzx ecx, cl
            add eax, ecx
            dec rdx
            test rdx, rdx
            jnz _memcmploop
        ret



section .text
    global acquire_lock
    global release_lock

    acquire_lock:
        lock bts dword [rdi], 0
        jc .spin
        ret
        .spin:
            pause
            test dword [rdi], 1
            jnz .spin
            jmp acquire_lock
    
    release_lock:
        mov dword [rdi], 0
        ret

.data
    syscall_table:
        .space 4096, 0

.text

    .globl register_syscall
    .globl syscall_int

    register_syscall:
        mov rax, rdi
        mov rdi, 8
        mul rdi
        mov [syscall_table+rax], rsi
        ret


    syscall_int:
        
        push rbx
        push rcx
        push rdx
        push rsi
        push rdi
        push rbp
        push rsp
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15
        
        push rdx
        mov rbx, 8
        mul rbx
        pop rdx

        pop rbx
        push rbx

        mov rax, [syscall_table+rax]
        call rax

        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rsp
        pop rbp
        pop rdi
        pop rsi
        pop rdx
        pop rcx
        pop rbx

        iretq


        
    
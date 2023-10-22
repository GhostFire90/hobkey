ALI equ 1<<0
MEMINFO equ 1<<1
FLAGS equ ALI | MEMINFO
MAGIC equ 0x1BADB002
CHECKSUM equ -(MAGIC + FLAGS)

section .multiboot
    ALIGN 4
    dd MAGIC
    dd FLAGS
    dd CHECKSUM

section .bss
    ALIGN 16
    stack_bottom:
        TIMES 16384 db
    stack_top:

section .text
    extern kernel_main
    global _start
    _start:
        mov esp, $stack_top
        call kernel_main
        cli
        hlt
        jmp 1b

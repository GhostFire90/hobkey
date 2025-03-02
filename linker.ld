OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

ENTRY(_start)

/* Define the program headers we want so the bootloader gives us the right */
/* MMU permissions */
PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    dynamic PT_DYNAMIC FLAGS((1 << 1) | (1 << 2)) ; /* Dynamic PHDR for relocations */
}

SECTIONS{
    . = 0xffffffff80000000;
    .text ALIGN(4K) : {
        *(.text .text.*)
    } :text
    .rodata ALIGN(4K) : {
        *(.rodata .rodata.*)
    } :rodata
    .data ALIGN(4K) : {
        *(.data .data.*)
    } :data

    .dynamic ALIGN(4K) : {
        *(.dynamic)
    } :data :dynamic

    .bss ALIGN(4K) : {
        *(.bss .bss.*)
        *(COMMON)
    } :data
    
    /* Discard .note.* and .eh_frame since they may cause issues on some hosts. */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
    __KERNEL_END__ = .;
}
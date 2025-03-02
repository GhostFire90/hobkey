use core::{include_str, arch::global_asm};

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("gdt.s"));
global_asm!(include_str!("idt.s"));
global_asm!(include_str!("syscalls.s"));
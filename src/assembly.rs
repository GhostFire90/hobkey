use core::{include_str, arch::global_asm};

global_asm!(include_str!("boot.s"));
global_asm!(include_str!("gdt.s"));
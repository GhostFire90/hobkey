use core::arch::asm;

pub fn inb(port : u16) -> u8{
    let mut ret : u8;
    unsafe{
        asm!(
            
            "in al, dx",
            in("dx") port,
            out("al") ret
        )
    }
    ret
}
pub fn outb(port : u16, byte : u8){
    unsafe {
        asm!(
            "OUT dx, al",
            in("dx") port,
            in("al") byte
        )
    }
}

pub fn inl(port : u16) -> u32{
    let mut ret : u32;
    unsafe {
        asm!(
            "in eax, dx",
            in("dx") port,
            out("eax") ret
        )
    }
    ret
}

pub fn outl(port : u16, data : u32){
    unsafe{
        asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") data
        )
    }
}
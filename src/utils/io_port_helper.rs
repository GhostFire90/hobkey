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
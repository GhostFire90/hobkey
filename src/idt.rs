
#[repr(C, packed)]
struct InterruptDescriptor64 {
    _offset_1        : u16, // offset bits 0..15
    _selector        : u16, // a code segment selector in GDT or LDT
    _ist             : u8,  // bits 0..2 holds Interrupt Stack Table offset, rest of bits zero.
    _type_attributes : u8,  // gate type, dpl, and p fields
    _offset_2        : u16, // offset bits 16..31
    _offset_3        : u32, // offset bits 32..63
    _zero            : u32, // reserved
}

pub const GATE_INTERRUPT : u8 = 0x0E;
#[allow(dead_code)]
pub const GATE_TRAP      : u8 = 0x0F;
pub const GATE_PRESENT   : u8 = 0x80;



extern "x86-interrupt" { 
    fn empty_int(); 
}

extern "sysv64" {
    fn get_idtr() -> *mut InterruptDescriptor64;
    fn refresh_idt(); 
}

impl InterruptDescriptor64{
    pub fn new(callback : u64, inttype : u8) -> Self{
        InterruptDescriptor64{
            _offset_1: (0xFFFF & callback) as u16,
            _offset_2: (0xFFFF & (callback >> 16)) as u16,
            _offset_3: (0xFFFFFFFF & (callback >> 32)) as u32,
            _type_attributes: inttype,
            _selector: 0x08,
            _ist: 0,
            _zero: 0
        }
    }
}

#[no_mangle]
#[allow(private_interfaces)]
pub extern "sysv64" fn IDTR_init(idtr : *mut InterruptDescriptor64){
    for i in 0..256{
        unsafe {
            idtr.add(i).write(InterruptDescriptor64::new(empty_int as u64, GATE_PRESENT | GATE_INTERRUPT))
        }
    }
}


pub fn set_interrupt(vector : u8, inttype : u8, callback : unsafe extern "x86-interrupt" fn() -> ()){
    let idtr = unsafe { get_idtr() };
    unsafe {
        idtr.add(vector as usize).write(InterruptDescriptor64::new(callback as u64, inttype));
        refresh_idt();
    }
}

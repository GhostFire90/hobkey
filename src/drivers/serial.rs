use crate::utils::{inb, outb};
pub struct Serial{
    port : u16
}

#[derive(Debug, Clone, Copy)]
pub enum SerialError{
    Faulty
}

pub const COM0 : u16 = 0x3F8;

impl Serial{

    pub const fn new_uninit(port : u16) -> Self{
        Self{port}
    } 
    pub fn initialize_port(port : u16) -> Result<(), SerialError>{
        outb(port + 1, 0x00);    // Disable all interrupts
        outb(port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(port + 1, 0x00);    //                  (hi byte)
        outb(port + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
        outb(port + 4, 0x1E);    // Set in loopback mode, test the serial chip

        outb(port + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(port + 0) != 0xAE  {
            Err(SerialError::Faulty)
        }
        else{
            outb(port+4, 0x0f);
            Ok(())
        }
    }

    

    pub fn new(port : u16) -> Result<Self, SerialError>{
        let res = Self::initialize_port(port);
        match res{
            Ok(_) => Ok(Self { port }),
            Err(x) => Err(x),
        }
    }
    pub fn data_ready(&self) -> bool{
        inb(self.port+5) & 1 != 0
    }
    pub fn read(&self) -> u8{
        while !self.data_ready(){};
        inb(self.port)
    }

    pub fn tx_empty(&self) -> bool{
        inb(self.port+5) & 0x20 != 0
    }
    pub fn write(&self, byte : u8){
        while !self.tx_empty(){}
        outb(self.port, byte);
    }
}
impl core::fmt::Write for Serial{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for x in s.bytes(){
            self.write(x);
        }
        Ok(())
    }
}
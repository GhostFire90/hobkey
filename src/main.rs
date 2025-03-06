#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]

mod assembly;
mod kernel;
mod limine_req;
mod memory;
mod spinlock;
mod idt;
mod ustar;
mod helpers;
mod syscalls;
mod utils;
mod drivers;


use core::{fmt, panic::PanicInfo};

use drivers::serial::{self, Serial};
use spinlock::Spinlock;

static PHANDLER_SERIAL : Spinlock<Serial> = Spinlock::new(Serial::new_uninit(serial::COM0));

#[panic_handler]
fn phandler(inf : &PanicInfo<'_>) ->!{
    
    fmt::write(PHANDLER_SERIAL.lock().get_mut(), format_args!("{}", inf)).unwrap();
    loop {
        
    }
}


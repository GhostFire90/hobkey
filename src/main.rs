#![no_std]
#![no_main]

mod assembly;
mod kernel;
mod limine_req;
mod memory;
mod spinlock;

use core::panic::PanicInfo;

#[panic_handler]
fn phandler(_ : &PanicInfo<'_>) ->!{
    
    loop {
        
    }
}


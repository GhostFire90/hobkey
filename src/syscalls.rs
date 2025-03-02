use crate::memory::paging::PageTableManager;
use crate::idt::{set_interrupt, GATE_PRESENT, GATE_INTERRUPT};
use crate::memory::pmm::PMM;

extern "sysv64" {
    fn register_syscall(num : u8, func : *const ());
}
extern "x86-interrupt"{
    fn syscall_int() -> ();
}

pub fn syscalls_initialize(){
    const SYSCALLS : [*const (); 3] = [
        PageTableManager::map as *const (),
        PMM::pop_page as *const (),
        PMM::push_page as *const ()
    ];
    SYSCALLS.iter().cloned().enumerate().for_each(|(i, x)| unsafe {
        register_syscall(i as u8, x);
    });
    set_interrupt(0x80, GATE_PRESENT | GATE_INTERRUPT, syscall_int);
}

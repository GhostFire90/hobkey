use crate::idt::{set_interrupt, InterruptStackFrame, GATE_INTERRUPT, GATE_PRESENT};
use crate::memory::paging::PageTableManager;
use crate::memory::pmm::PMM;

extern "sysv64" {
  fn register_syscall(num: u8, func: *const ());
}
extern "x86-interrupt" {
  fn syscall_int(_: InterruptStackFrame) -> ();
}

pub fn syscalls_initialize()
{
  const SYSCALLS: [*const (); 3] = [
    PageTableManager::map as *const (),
    PMM::pop_page as *const (),
    PMM::push_page as *const (),
  ];
  SYSCALLS
    .iter()
    .cloned()
    .enumerate()
    .for_each(|(i, x)| unsafe {
      register_syscall(i as u8, x);
    });
  set_interrupt(0x80, GATE_PRESENT | GATE_INTERRUPT, syscall_int);
}

#[macro_export]
macro_rules! syscall {
  ($num:expr) => {
    {let ret: u64;
    unsafe
    {
      core::arch::asm!(
        "int 0x80",
        in("rax") $num,
        lateout("rax") ret,
      );
    }
    ret
  }};
  (
    $num:expr,
    $a1:expr
  ) =>{{
    let ret : u64;
    unsafe{core::arch::asm!(
      "int 0x80",
      in("rax") $num,
      in("rdi") $a1,
      lateout("rax") ret,
    );}
    ret
  }};
   (
    $num:expr,
    $a1:expr,
    $a2:expr
  ) =>{{
    let ret : u64;
    unsafe{core::arch::asm!(
      "int 0x80",
      in("rax") $num,
      in("rdi") $a1,
      in("rsi") $a2,
      lateout("rax") ret,
    );}
    ret
  }};
  (
    $num:expr,
    $a1:expr,
    $a2:expr,
    $a3:expr
  ) =>{{
    let ret : u64;
    unsafe{core::arch::asm!(
      "int 0x80",
      in("rax") $num,
      in("rdi") $a1,
      in("rsi") $a2,
      in("rdx") $a3,
      lateout("rax") ret,
    );}
    ret
  }};
  (
    $num:expr,
    $a1:expr,
    $a2:expr,
    $a3:expr,
    $a4:expr
  ) =>{{
    let ret : u64;
    unsafe{core::arch::asm!(
      "int 0x80",
      in("rax") $num,
      in("rdi") $a1,
      in("rsi") $a2,
      in("rdx") $a3,
      in("rcx") $a4,
      lateout("rax") ret,
    );}
    ret
  }};
  (
    $num:expr,
    $a1:expr,
    $a2:expr,
    $a3:expr,
    $a4:expr,
    $a5:expr
  ) =>{{
    let ret : u64;
    unsafe{core::arch::asm!(
      "int 0x80",
      in("rax") $num,
      in("rdi") $a1,
      in("rsi") $a2,
      in("rdx") $a3,
      in("rcx") $a4,
      in("r8")  $a5,
      lateout("rax") ret,
    );}
    ret
  }};
  (
    $num:expr,
    $a1:expr,
    $a2:expr,
    $a3:expr,
    $a4:expr,
    $a5:expr,
    $a6:expr
  ) =>{{
    let ret : u64;
    unsafe{
      core::arch::asm!(
        "int 0x80",
        in("rax") $num,
        in("rdi") $a1,
        in("rsi") $a2,
        in("rdx") $a3,
        in("rcx") $a4,
        in("r8")  $a5,
        in("r9")  $a6,
        lateout("rax") ret,
      );
    }
    ret
  }}
}

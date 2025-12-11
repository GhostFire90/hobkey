use core::ffi::CStr;
use core::fmt::{self, Write};

use crate::drivers::serial::{self, Serial};
use crate::limine_req::{FB_REQ, HHDM_REQ, MODULE_REQ};
use crate::memory::paging::{paging_flags, PageTableManager};
use crate::memory::pmm::PMM;
use crate::syscalls;
use crate::ustar;

fn cstrcmp(a: &[u8], b: &[u8]) -> bool
{
  for (ac, bc) in a.iter().zip(b.iter())
  {
    if *ac != *bc
    {
      return false;
    }
  }
  true
}

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  let mut serial = Serial::new(serial::COM0).unwrap();

  fmt::write(
    &mut serial,
    format_args!(
      "\n+-----------------------+\n|Beginning of Hobkey Log|\n+-----------------------+\n"
    ),
  )
  .expect("Couldnt write to serial");

  let fbr = FB_REQ.get_response().unwrap();
  let fb = fbr.framebuffers().next().unwrap();

  let buf_len: usize = ((fb.bpp() as u64 / 8) * fb.width() * fb.height())
    .try_into()
    .unwrap();

  let modules = MODULE_REQ.get_response().unwrap().modules();
  let initrd_mod = modules
    .iter()
    .find(|x| cstrcmp(x.path().to_bytes(), "/boot/initrd.tar".as_bytes()))
    .unwrap();
  let hhdm_offset = HHDM_REQ.get_response().unwrap().offset();
  let initrd_addr = initrd_mod.addr() as u64;
  let initrd_phy = initrd_addr - hhdm_offset;
  let initrd_size = initrd_mod.size().next_multiple_of(4096);

  let fb_addr = fb.addr() as u64;
  let fb_phy = fb_addr - hhdm_offset;

  serial
    .write_fmt(format_args!("INITRD_ADDR 0x{:x}\n", initrd_addr))
    .unwrap();

  PMM::init();
  PMM::reclaim_bootloader().unwrap();
  PageTableManager::init().unwrap();
  PageTableManager::map_range(
    initrd_phy..=initrd_phy + initrd_size,
    initrd_addr..=initrd_addr + initrd_size,
    paging_flags::PAGING_PRESENT | paging_flags::PAGING_R,
  )
  .unwrap();
  PageTableManager::map_range(
    fb_phy..=fb_phy + buf_len as u64,
    fb_addr..=fb_addr + buf_len as u64,
    paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT,
  )
  .unwrap();
  serial
    .write_fmt(format_args!("FRAME_BUFF ADDR 0x{:x}\n", fb_addr))
    .unwrap();
  syscalls::syscalls_initialize();

  let _f = ustar::find_file("./test.txt", initrd_addr as *const u8, initrd_size as usize).unwrap();
  serial
    .write_str(
      unsafe { CStr::from_ptr((initrd_addr as usize + _f.1) as *const i8) }
        .to_str()
        .unwrap(),
    )
    .unwrap();

  loop
  {}
}

use core::ffi::CStr;
use core::fmt::{self, Write};

use alloc::vec::Vec;

use crate::drivers::pci::pci_communication::PciDevice;
use crate::drivers::serial::{self, Serial};
use crate::limine_req::{FB_REQ, HHDM_REQ, MODULE_REQ};
use crate::memory::mmap::MmapTracker;
use crate::memory::paging::{paging_flags, PageTableManager};
use crate::memory::pmm::PMM;
use crate::memory::HHDM_OFFSET;
use crate::process::{Process, CURRENT_PROC};
use crate::spinlock::Spinlock;
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
  let hhdm_offset = HHDM_OFFSET.get();
  let mut initrd_addr = initrd_mod.addr() as u64;
  let initrd_phy = initrd_addr - hhdm_offset;
  let initrd_size = initrd_mod.size().next_multiple_of(4096);

  let mut fb_addr = fb.addr() as u64;
  let fb_phy = fb_addr - hhdm_offset;

  syscalls::syscalls_initialize();

  PMM::init();
  PMM::reclaim_bootloader().unwrap();
  let ptm = PageTableManager::new_bootstrap().unwrap();
  let mmap_start = ptm.mmap_start;
  let mmap_end = ptm.mmap_end;

  let kernel_proc = Process::new(ptm);
  let mut proc_guard = CURRENT_PROC.lock();
  proc_guard.replace(kernel_proc);
  drop(proc_guard);

  let mmap_tracker = MmapTracker::new(mmap_start, mmap_end);

  proc_guard = CURRENT_PROC.lock();
  let proc = proc_guard.as_mut().unwrap();

  proc
    .ptm_operation(|ptm| {
      ptm.mmap_tracker.lock().replace(mmap_tracker);
      initrd_addr = ptm
        .map_into(
          initrd_phy,
          initrd_size as usize,
          paging_flags::PAGING_PRESENT,
        )
        .unwrap();
      fb_addr = ptm
        .map_into(
          fb_phy,
          buf_len,
          paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT,
        )
        .unwrap();
      Ok(())
    })
    .unwrap();

  serial
    .write_fmt(format_args!("FRAME_BUFF ADDR 0x{:x}\n", fb_addr))
    .unwrap();
  serial
    .write_fmt(format_args!("INITRD_ADDR 0x{:x}\n", initrd_addr))
    .unwrap();

  let _f = ustar::find_file("./test.txt", initrd_addr as *const u8, initrd_size as usize).unwrap();
  serial
    .write_str(
      unsafe { CStr::from_ptr((initrd_addr as usize + _f.1) as *const i8) }
        .to_str()
        .unwrap(),
    )
    .unwrap();

  for bus in 0..255
  {
    for device in 0..32
    {
      let dev = PciDevice::new(bus, device, 0);
      if dev.exists()
      {
        serial
          .write_fmt(format_args!(
            "Device at {bus}:{device}: {:?}\n",
            dev.get_device_type()
          ))
          .unwrap();
      }
    }
  }

  loop
  {}
}

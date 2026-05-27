use core::fmt::{self, Write};
use core::ptr::NonNull;

use crate::drivers::acpi::{Xsdp, Xsdt, RSDP};

use crate::drivers::pci::pci_communication::PciDeviceAddress;
use crate::drivers::serial::{self, Serial};
use crate::limine_req::{FB_REQ, MODULE_REQ, RSDP_REQ};
use crate::memory::mmap::MmapTracker;
use crate::memory::paging::paging_flags::{PAGING_PRESENT, PAGING_RW};
use crate::memory::paging::{paging_flags, PageTableManager, VirtualAddress};
use crate::memory::pmm::PMM;
use crate::memory::HHDM_OFFSET;
use crate::process::{Process, CURRENT_PROC};
use crate::psf::Psf;
use crate::syscalls;
use crate::timers::apic::MADT;
use crate::ustar::UstarArchive;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  let mut serial = Serial::new(serial::COM0).unwrap();
  // gotta figure this out lmao
  RSDP.get_or_init(|| {
    let response = RSDP_REQ.get_response().unwrap().address();
    let p_xsdp = response as *mut Xsdp;
    unsafe {
      let xsdp = *p_xsdp;
      (
        response as VirtualAddress - HHDM_OFFSET.get(),
        xsdp.length as usize,
      )
    }
  });

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
    .find(|x| x.path().to_bytes().eq("/boot/initrd.tar".as_bytes()))
    .unwrap();
  let hhdm_offset = HHDM_OFFSET.get();
  let mut initrd_addr = initrd_mod.addr() as u64;
  let initrd_phy = initrd_addr - hhdm_offset;
  let initrd_size = initrd_mod.size().next_multiple_of(4096);

  let mut fb_addr = fb.addr() as u64;
  let fb_phy = fb_addr - hhdm_offset;
  let fb_stride = fb.pitch();

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
  let mut xsdp = None;

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
      xsdp = NonNull::new({
        let (addr, len) = RSDP.get().unwrap().clone();

        (ptm.map_into(addr, len, PAGING_RW | PAGING_PRESENT)?) as *mut Xsdp
      });

      Ok(())
    })
    .unwrap();
  drop(proc_guard);

  Xsdt::new(xsdp.unwrap()).iter().for_each(|x| {
    serial
      .write_fmt(format_args!("XSDT at: 0x{:x}\n", x.addr()))
      .unwrap();
    unsafe {
      serial
        .write_fmt(format_args!(
          "APIC at: {:p}\n",
          x.as_ref().find_table::<MADT>("APIC").unwrap().as_ptr()
        ))
        .unwrap()
    };
  });

  serial
    .write_fmt(format_args!("FRAME_BUFF ADDR 0x{:x}\n", fb_addr))
    .unwrap();
  serial
    .write_fmt(format_args!("INITRD_ADDR 0x{:x}\n", initrd_addr))
    .unwrap();

  let initrd_addr = NonNull::new(initrd_addr as *mut u8).expect("Initrd address is NULL");
  let ustar = UstarArchive::new(initrd_addr, initrd_size as usize);

  for (header, _) in ustar.iter()
  {
    let (name_bytes, len) = header.file_name();
    let name = str::from_utf8(&name_bytes[0..len]).unwrap();
    serial.write_fmt(format_args!("File: {}\n", name)).unwrap();
  }

  let (_, psf_file) = ustar.iter().find_file("./resources/zap-vga.psf").unwrap();
  let psf = Psf::new(NonNull::new(psf_file.as_ptr() as *mut u8).unwrap()).unwrap();

  serial
    .write_fmt(format_args!(
      "height {} count {}\n",
      psf.height(),
      psf.get_glyph_count()
    ))
    .unwrap();

  let fb =
    unsafe { core::slice::from_raw_parts_mut(fb_addr as *mut u32, buf_len / size_of::<u32>()) };
  let glyph_width = psf.width();

  let mut x_offset = 0;
  for c in 0..127
  {
    let glyph = psf.get_glyph(c.into()).unwrap().expand(u32::MAX, 0);
    for i in 0..glyph.len()
    {
      let row = i / glyph_width;
      let col = i % glyph_width;
      fb[row * (fb_stride as usize / 4) + col + x_offset] = glyph[i];
    }
    x_offset += glyph_width;
  }

  for bus in 0..255
  {
    for device in 0..32
    {
      let dev = PciDeviceAddress::new(bus, device, 0).unwrap();
      if dev.exists()
      {
        serial
          .write_fmt(format_args!(
            "Device at {bus}:{device}: {:?} Multi-function? {}\n",
            dev.get_device_type(),
            dev.is_multi_function()
          ))
          .unwrap();
      }
    }
  }

  loop
  {}
}

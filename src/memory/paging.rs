#![allow(dead_code)]

use super::pmm::{PhysicalAddress, PmmError, PMM};
use super::PAGE_SIZE;
use crate::drivers::serial::{Serial, COM0};
use crate::helpers::map_phy_temp;
use crate::limine_req::{HHDM_REQ, KERNEL_REQ};
use crate::memory::alloc::KALLOC;
use crate::memory::mmap::MmapTracker;
use crate::memory::{get_containing_page, MemoryRange, HHDM_OFFSET};
use crate::process::CURRENT_PROC;
use crate::{limine_req, spinlock::*};
use core::arch::{asm, global_asm};
use core::cell::LazyCell;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::ops::{BitXor, Range, RangeInclusive, Shl};
use core::sync::atomic::AtomicBool;
use core::{fmt, include_str, usize};

global_asm!(include_str!("paging.s"));

extern "sysv64" {
  fn invalidate_page(pg: u64);
}

fn set_cr3(pml4: PhysicalAddress)
{
  unsafe {
    asm!(
        "mov cr3, {0}",
        in(reg) pml4
    )
  }
}

const TABLE_ENTRY_COUNT: usize = 512;

const HIGHER_HALF_ADDR: VirtualAddress = VirtualAddress::MAX ^ ((1 << 47) - 1);

extern "C" {
  static MAX_PHY_BIT: u8;
  static MAX_VRT_BIT: u8;
  static __KERNEL_END__: VirtualAddress;
  static STACK_TOP: VirtualAddress;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct TableEntry(u64);

#[repr(usize)]
pub enum TableLayer
{
  PML4,
  PDPT,
  PDT,
  PT,
}

pub mod paging_flags
{

  /// Present; must be 1 to reference a paging table
  pub const PAGING_PRESENT: u64 = 1 << 0;

  /// Read/write; if 0, writes may not be allowed (see Section 4.6)
  pub const PAGING_RW: u64 = 1 << 1;

  /// User/supervisor; if 0, user-mode accesses are not allowed (see Section 4.6)
  pub const PAGING_USER: u64 = 1 << 2;

  /// Page-level write-through; indirectly determines memory type (see Section 4.9.2)
  pub const PAGING_PWT: u64 = 1 << 3;

  /// Page-level cache disable; indirectly determines memory type (see Section 4.9.2)
  pub const PAGING_PCD: u64 = 1 << 4;

  /// Accessed; indicates whether this entry has been used (see Section 4.8)
  pub const PAGING_ACCESSED: u64 = 1 << 5;

  /// For ordinary paging, ignored; for HLAT paging, restart (see Section 4.8)
  pub const PAGING_R: u64 = 1 << 11;
}

/// TODO, probably need to fix this lmao
const KERNEL_START: VirtualAddress = 0xffffffff80000000;

pub struct PageTableManager
{
  phy_addr: PhysicalAddress,
  tmp_address: VirtualAddress,
  tmp_entry_address: VirtualAddress,

  pub mmap_start: VirtualAddress,
  pub mmap_end: VirtualAddress,
  pub mmap_tracker: Spinlock<Option<MmapTracker>>,
}

pub type VirtualAddress = u64;

static PAGE_TABLE_MANAGER: Spinlock<Option<PageTableManager>> = Spinlock::new(None);
pub static USE_HHDM: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Clone)]
pub enum PtmError
{
  NoMapping(VirtualAddress),
  InvalidRange,
  IncorrectPageSize,
  UnallignedPage,
  PmmError(PmmError),
  PtmNotReady,
}

fn split_virtual(virt: VirtualAddress) -> ([u16; 4], u16)
{
  (
    [
      (virt >> 39) as u16 & 0x1ff,
      (virt >> 30) as u16 & 0x1ff,
      (virt >> 21) as u16 & 0x1ff,
      (virt >> 12) as u16 & 0x1ff,
    ],
    virt as u16 & 0x7ff,
  )
}

fn next_page(addr: u64) -> u64
{
  // if addr is not the start of a page, go to next multiple of PAGE_SIZE
  // otherwise set next multiple
  addr.next_multiple_of(PAGE_SIZE)
}

fn sign_extend(i: VirtualAddress) -> VirtualAddress
{
  let sign = i & unsafe { 1 << MAX_PHY_BIT };
  if sign != 0
  {
    let mask = ((0 - 1) as u64) ^ (unsafe { (1 << MAX_PHY_BIT - 1) - 1 });
    i | mask
  }
  else
  {
    i
  }
}

fn from_indexes(indexes: ([u16; 4], u16)) -> VirtualAddress
{
  /*
  ([
      (virt >> 39) as u16 & 0x1ff,
      (virt >> 30) as u16 & 0x1ff,
      (virt >> 21) as u16 & 0x1ff,
      (virt >> 12) as u16 & 0x1ff,

  ], virt as u16 & 0xeff)
   */
  (indexes.0[0] as u64).shl(39)
    | (indexes.0[1] as u64).shl(30)
    | (indexes.0[2] as u64).shl(21)
    | (indexes.0[3] as u64).shl(12)
    | (indexes.1 as u64)
}

impl PageTableManager
{
  fn dump_to_serial(&mut self, top: PhysicalAddress)
  {
    let mut serial = Serial::new_uninit(COM0);

    fn ds_recurse(
      ptm: &mut PageTableManager,
      layer: VirtualAddress,
      serial: &mut Serial,
      depth: usize,
    )
    {
      if layer == 0
      {
        return;
      }
      let entries = layer as *mut TableEntry;
      serial.write(b'[');
      let mut previous = false;
      for i in 0..512
      {
        let entry = unsafe { entries.add(i).read() };
        match entry.get_pointer()
        {
          0 => continue,
          x =>
          {
            if previous
            {
              serial.write(b',');
            }
            serial.write(b'{');
            let old_tmp = ptm.get_temp();
            let next = ptm.map_temp(x).unwrap_or(0);
            fmt::write(
              serial,
              format_args!(
                "\"flags\":{:#b},\"address\":\"{:x}\",\"depth\":{}",
                entry.get_flags(),
                entry.get_pointer(),
                depth
              ),
            )
            .unwrap();
            if depth != TableLayer::PT.into()
            {
              fmt::write(serial, format_args!(",\"layer\":")).unwrap();
              ds_recurse(ptm, next, serial, depth + 1);
            }
            ptm.map_temp(old_tmp).unwrap();
            serial.write(b'}');
            previous = true;
          }
        }
      }
      serial.write(b']');
    }
    let old_tmp = self.get_temp();
    let next = self.map_temp(top).unwrap_or(0);
    ds_recurse(self, next, &mut serial, 0);
    serial.write(b'\n');
    self.map_temp(old_tmp).unwrap();
  }

  pub(crate) fn new_bootstrap() -> Result<Self, PtmError>
  {
    let _ = Serial::new_uninit(COM0).write_fmt(format_args!(
      "TOP INDEX: {}\n",
      split_virtual(u64::MAX.bitxor(1u64.shl(47) - 1u64)).0[0]
    ));

    let pg = PMM::pop_page();

    let phy_addr = pg;

    // Get Address of the kernel in memory
    let kaddr = KERNEL_REQ.get_response().unwrap();

    // Get the next closest page after the end of the kernel to make sure we dont truncate the kernel
    let kend = next_page(core::ptr::addr_of!(__KERNEL_END__) as VirtualAddress);

    // this is the address that will be given to the user when they call map_temp
    let tmp_space = kend + PAGE_SIZE;

    let tmp_page = tmp_space + PAGE_SIZE;

    let tmp_space_idx = split_virtual(tmp_space).0[3];
    let tmp_address = tmp_space;
    let tmp_entry_address = tmp_page + (tmp_space_idx as usize * size_of::<TableEntry>()) as u64;

    // The kernels virtual and physical addresses
    let kernel_start = kaddr.virtual_base();
    let kernel_start_phy = kaddr.physical_base();

    let mut ptm = Self {
      phy_addr,
      tmp_address,
      tmp_entry_address,
      mmap_tracker: Spinlock::new(None),
      mmap_start: HIGHER_HALF_ADDR,
      mmap_end: kernel_start,
    };

    // kend is in virtual space so make sure to get difference between it and the virtual
    let klen = kend - kernel_start;
    ptm.map_range(
      kernel_start_phy..=kernel_start_phy + klen,
      kernel_start..=kernel_start + klen,
      paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT,
    )?;

    let tmp_space_pdt = ptm.crawl(tmp_space, TableLayer::PDT)?;

    // which means we need to make it modifiable without re-mapping or else we get crazy infinite recurrsion
    // we will use the next virtual page to map to the tmp space

    let _hhdm_offs = HHDM_OFFSET.get();

    let ptmp_page_entry = ptm.crawl_alloc(
      tmp_page,
      TableLayer::PT,
      paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT,
    );
    unsafe {
      let mut tmp_page_entry = ptmp_page_entry.read();
      let ptr = tmp_space_pdt.read().get_pointer();
      tmp_page_entry.set_pointer(ptr);
      tmp_page_entry.set_flags(paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
      ptmp_page_entry.write(tmp_page_entry);
    }

    // ptm.dump_to_serial(pg);

    set_cr3(phy_addr);

    USE_HHDM.store(false, core::sync::atomic::Ordering::Release);

    let heap_base = tmp_page + PAGE_SIZE;
    KALLOC.set_base(heap_base);
    KALLOC.set_size((u64::MAX - heap_base) as usize);

    ptm.dump_to_serial(pg);

    Ok(ptm)
  }

  pub fn mmap(&mut self, length: usize, flags: u64) -> Option<VirtualAddress>
  {
    let mut length = length.next_multiple_of(PAGE_SIZE as usize);
    let mapping = self
      .mmap_tracker
      .lock()
      .as_mut()
      .unwrap()
      .get_mapping(length)?;
    let mut current = mapping.0.start as VirtualAddress;
    let ret = current;
    let mut o_self = Some(self);
    while length > 0
    {
      let pg = PMM::pop_page_with_ptm(&mut o_self);
      o_self = o_self.and_then(|x| {
        x.map(pg, current, flags).unwrap();
        Some(x)
      });
      current += PAGE_SIZE;
      length -= PAGE_SIZE as usize;
    }
    Some(ret)
  }

  pub fn munmap(&mut self, addr: VirtualAddress, length: usize)
  {
    let page = get_containing_page(addr);
    let offset = addr - page;
    let length = (length as u64 + offset).next_multiple_of(PAGE_SIZE);

    let mapping = page..page + length;

    self.unmap_range(page..=page + length, true).unwrap();

    self
      .mmap_tracker
      .lock()
      .as_mut()
      .unwrap()
      .release_mapping(mapping.into());
  }

  pub fn map_into(
    &mut self,
    phy_addr: PhysicalAddress,
    length: usize,
    flags: u64,
  ) -> Option<VirtualAddress>
  {
    let page = get_containing_page(phy_addr);
    let offset = phy_addr - page;
    let length = (length + offset as usize).next_multiple_of(PAGE_SIZE as usize);
    let mapping: Range<VirtualAddress> = self
      .mmap_tracker
      .lock()
      .as_mut()
      .unwrap()
      .get_mapping(length)?
      .into();
    let ret = mapping.start;
    self
      .map_range(
        page..=page + length as u64,
        mapping.start..=mapping.end,
        flags,
      )
      .ok()?;
    Some(ret)
  }

  pub fn new(
    &mut self,
    mmap_start: VirtualAddress,
    mmap_end: VirtualAddress,
  ) -> Result<Self, PtmError>
  {
    let p_higher_half = self.crawl(HIGHER_HALF_ADDR, TableLayer::PML4)?;
    let higher_half = unsafe { *p_higher_half };
    let (tmp_address, tmp_entry_address) = (self.tmp_address, self.tmp_entry_address);
    let phy_addr = PMM::pop_page_with_ptm(&mut Some(self));
    let mut ret = Self {
      phy_addr,
      tmp_address,
      tmp_entry_address,
      mmap_tracker: Spinlock::new(None), // TODO! Figure out what to do here lmao
      mmap_start,
      mmap_end,
    };

    unsafe {
      ret
        .crawl(HIGHER_HALF_ADDR, TableLayer::PML4)?
        .write(higher_half);
    };
    Ok(ret)
  }

  pub fn map(
    &mut self,
    phy_addr: PhysicalAddress,
    virt_addr: VirtualAddress,
    flags: u64,
  ) -> Result<(), PtmError>
  {
    let pentry = self.crawl_alloc(virt_addr, TableLayer::PT, flags);
    let mut entry = unsafe { pentry.read() };
    entry.set_pointer(phy_addr);
    entry.set_flags(flags);

    unsafe {
      invalidate_page(virt_addr);
      pentry.write(entry);
    };
    Ok(())
  }

  pub extern "sysv64" fn map_interrupt(
    phy_addr: PhysicalAddress,
    virt_addr: VirtualAddress,
    flags: u64,
  ) -> i8
  {
    if phy_addr % PAGE_SIZE != 0
    {
      -1
    }
    else
    {
      // let mut guard = PAGE_TABLE_MANAGER.lock();
      // let ptm = guard.get_mut();
      let mut guard = CURRENT_PROC.lock();
      let proc = guard.as_mut().unwrap();

      proc
        .ptm_operation(|ptm| ptm.map(phy_addr, virt_addr, flags))
        .unwrap();
      0
    }
  }

  pub fn unmap(&mut self, virt_addr: VirtualAddress, free_phy: bool) -> Result<(), PtmError>
  {
    self
      .crawl(virt_addr, TableLayer::PT)
      .and_then(|entry| unsafe {
        let ret = (*entry).get_pointer();
        (*entry).set_flags(0);
        if free_phy
        {
          PMM::push_page_with_ptm(ret, self);
        }
        Ok(())
      })
  }

  pub extern "sysv64" fn unmap_interrupt(virt_addr: VirtualAddress, free_phy: bool) -> i8
  {
    if virt_addr % PAGE_SIZE != 0
    {
      return -1;
    }

    let mut guard = CURRENT_PROC.lock();
    let proc = guard.as_mut().unwrap();

    proc
      .ptm_operation(|ptm| ptm.unmap(virt_addr, free_phy))
      .map(|_| 0)
      .map_err(|_| -1)
      .unwrap()
  }

  pub fn unmap_range(
    &mut self,
    virt: RangeInclusive<VirtualAddress>,
    free_phy: bool,
  ) -> Result<(), PtmError>
  {
    if (virt.end() - virt.start()) % PAGE_SIZE != 0
    {
      return Err(PtmError::IncorrectPageSize);
    }
    else if (virt.start() % PAGE_SIZE) != 0
    {
      return Err(PtmError::UnallignedPage);
    }
    for pg in virt.step_by(PAGE_SIZE as usize)
    {
      self.unmap(pg, free_phy)?;
    }
    Ok(())
  }

  pub fn map_range(
    &mut self,
    phy: RangeInclusive<PhysicalAddress>,
    virt: RangeInclusive<VirtualAddress>,
    flags: u64,
  ) -> Result<(), PtmError>
  {
    if phy.clone().count() != virt.clone().count()
    {
      Err(PtmError::InvalidRange)
    }
    else if (phy.end() - phy.start()) % PAGE_SIZE != 0
    {
      Err(PtmError::IncorrectPageSize)
    }
    else if phy.start() % PAGE_SIZE != 0
    {
      Err(PtmError::UnallignedPage)
    }
    else
    {
      let pi = phy.step_by(PAGE_SIZE as usize);
      let vi = virt.step_by(PAGE_SIZE as usize);
      for (p, v) in pi.zip(vi)
      {
        self.map(p, v, flags)?;
      }
      Ok(())
    }
  }
  pub fn map_temp(&mut self, phy_addr: PhysicalAddress) -> Result<VirtualAddress, PtmError>
  {
    if USE_HHDM.load(core::sync::atomic::Ordering::Acquire)
    {
      let hhdm = HHDM_OFFSET.get();

      return Ok(phy_addr + hhdm);
    }

    let ret = self.tmp_address;
    let ppte = self.tmp_entry_address as *mut TableEntry;

    unsafe {
      let mut pte = ppte.read();
      pte.set_pointer(phy_addr);
      pte.set_flags(paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
      ppte.write(pte);
      invalidate_page(ret);
    }
    Ok(ret)
  }
  // pub fn map_temp_old(phy_addr: PhysicalAddress) -> Result<VirtualAddress, PtmError>
  // {
  //   let mut guard = CURRENT_PROC.lock();

  //   if let Some(proc) = guard.as_mut()
  //   {
  //     if !proc.has_ptm()
  //     {
  //       let offset = HHDM_REQ.get_response().unwrap().offset();
  //       Ok(phy_addr + offset)
  //     }
  //     else
  //     {
  //       proc.ptm_operation(move |ptm| {
  //         let ret = ptm.tmp_address;
  //         let ppte = ptm.tmp_entry_address as *mut TableEntry;

  //         unsafe {
  //           let mut pte = ppte.read();
  //           pte.set_pointer(phy_addr);
  //           pte.set_flags(paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
  //           ppte.write(pte);
  //           invalidate_page(ret);
  //         }
  //         Ok(ret)
  //       })
  //     }
  //   }
  //   else
  //   {
  //     let offset = HHDM_REQ.get_response().unwrap().offset();
  //     Ok(phy_addr + offset)
  //   }
  // }
  pub fn unmap_temp()
  {
    let mut guard = CURRENT_PROC.lock();
    let proc = guard.as_mut().unwrap();
    let _ = proc.ptm_operation(|ptm| {
      let ppte = ptm.tmp_entry_address as *mut TableEntry;
      unsafe {
        let mut pte = ppte.read();
        pte.set_pointer(0);
        pte.set_flags(0);
      }
      Ok(())
    });
  }

  pub fn get_physical(virt_addr: VirtualAddress) -> Result<PhysicalAddress, PtmError>
  {
    let mut guard = CURRENT_PROC.lock();
    let proc = guard.as_mut().unwrap();
    let op = |ptm: &mut PageTableManager| {
      let res = ptm.crawl(virt_addr, TableLayer::PT);
      match res
      {
        Ok(x) =>
        {
          let entry = unsafe { x.read() };
          Ok(entry.get_pointer())
        }
        Err(e) => Err(e),
      }
    };

    match proc.ptm_operation(op)
    {
      Err(PtmError::PtmNotReady) =>
      {
        let offset = HHDM_OFFSET.get();

        Ok(virt_addr - offset)
      }
      e => e,
    }
  }
  pub fn get_temp(&self) -> PhysicalAddress
  {
    if USE_HHDM.load(core::sync::atomic::Ordering::Acquire)
    {
      0
    }
    else
    {
      let tmp = self.tmp_entry_address as *const TableEntry;
      unsafe { tmp.read().get_pointer() }
    }
  }

  pub(self) fn crawl(
    &mut self,
    virt: VirtualAddress,
    layer: TableLayer,
  ) -> Result<*mut TableEntry, PtmError>
  {
    let (indexes, _) = split_virtual(virt);
    let phy_addr = self.phy_addr;
    let mut o_ptm = Some(self);

    let mut ret = map_phy_temp(phy_addr, &mut o_ptm)? as *mut TableEntry;
    let l: usize = layer.into();

    for i in 0..l
    {
      let current = unsafe { ret.add(indexes[i] as usize).read() };
      let phy = current.get_pointer();
      if phy == 0
      {
        return Err(PtmError::NoMapping(virt));
      }
      ret = map_phy_temp(phy, &mut o_ptm)? as *mut TableEntry;
    }
    Ok(unsafe { ret.add(indexes[l].into()) })
  }

  pub(self) fn crawl_alloc(
    &mut self,
    virt: VirtualAddress,
    table_layer: TableLayer,
    flags: u64,
  ) -> *mut TableEntry
  {
    let (indexes, _) = split_virtual(virt);
    let phy_addr = self.phy_addr;

    let mut o_ptm = Some(self);

    let mut ret = map_phy_temp(phy_addr, &mut o_ptm).unwrap() as *mut TableEntry;
    let l: usize = table_layer.into();

    for i in 0..l
    {
      let mut entry = unsafe { ret.add(indexes[i] as usize).read() };
      entry.check_and_alloc(flags, &mut o_ptm);
      unsafe { ret.add(indexes[i] as usize).write(entry) }
      ret = map_phy_temp(entry.get_pointer(), &mut o_ptm).unwrap() as *mut TableEntry;
    }
    unsafe { ret.add(indexes[l].into()) }
  }
}
impl TableEntry
{
  pub const fn generate_phy_mask() -> u64
  {
    unsafe {
      let upper = (1 << (MAX_PHY_BIT - 1)) - 1;
      let lower = (1 << 11) - 1;
      upper ^ lower
    }
  }

  pub fn new(phy: PhysicalAddress, rw: bool) -> TableEntry
  {
    let phy_mask = Self::generate_phy_mask();
    let mut val = 0;
    val |= (rw as u64) << 1;
    val |= (phy << 12) & phy_mask;
    TableEntry(val)
  }
  pub fn get_pointer(&self) -> PhysicalAddress
  {
    let phy_mask = Self::generate_phy_mask();
    self.0 & phy_mask
  }

  pub fn set_pointer(&mut self, ptr: PhysicalAddress)
  {
    let phy_mask = Self::generate_phy_mask();
    self.0 &= !phy_mask;
    self.0 |= ptr & phy_mask;
  }

  pub fn get_flags(&self) -> u64
  {
    let phy_mask = Self::generate_phy_mask();
    self.0 & !phy_mask
  }

  pub fn set_flags(&mut self, flags: u64)
  {
    let phy_mask = Self::generate_phy_mask();
    self.0 &= phy_mask;
    self.0 |= flags;
  }
}

impl TableEntry
{
  pub fn check_and_alloc(&mut self, flags: u64, ptm: &mut Option<&mut PageTableManager>)
  {
    let present = (self.0 & paging_flags::PAGING_PRESENT) != 0;
    if present
    {
      self.set_flags(flags);
    }
    else
    {
      let page = PMM::pop_page_with_ptm(ptm);

      self.set_pointer(page);
      self.set_flags(flags);
    }
  }
}

impl From<TableEntry> for u64
{
  fn from(value: TableEntry) -> Self
  {
    value.0
  }
}

impl From<u64> for TableEntry
{
  fn from(value: u64) -> Self
  {
    Self(value)
  }
}

impl From<TableLayer> for usize
{
  fn from(value: TableLayer) -> Self
  {
    match value
    {
      TableLayer::PML4 => 0,
      TableLayer::PDPT => 1,
      TableLayer::PDT => 2,
      TableLayer::PT => 3,
    }
  }
}

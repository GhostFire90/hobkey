use core::{
  num::Wrapping,
  ptr::{addr_of, NonNull},
};

use crate::{
  memory::{
    get_containing_page,
    paging::{
      paging_flags::{PAGING_PRESENT, PAGING_RW},
      VirtualAddress,
    },
    pmm::PhysicalAddress,
    PAGE_SIZE,
  },
  process::CURRENT_PROC,
  spinlock::SpinlockOnce,
};

pub static RSDP: SpinlockOnce<(PhysicalAddress, usize)> = SpinlockOnce::new();

pub trait AcpiTable {}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct ACPISDTHeader
{
  signature: [u8; 4],
  length: u32,
  revision: u8,
  checksum: u8,
  oem_id: [u8; 6],
  oem_table_id: [u8; 8],
  oem_revision: u32,
  creator_id: u32,
  creator_revision: u32,
}

impl ACPISDTHeader
{
  pub fn validate_checksum(p_self: *const Self) -> bool
  {
    unsafe {
      let len = (*p_self).length;
      let ptr = p_self as *const i8;
      let data = core::slice::from_raw_parts(ptr, len as usize);
      let mut sum = Wrapping(0);
      for x in data
      {
        sum += x;
      }
      sum.0 == 0
    }
  }
}

#[repr(packed)]
pub struct Xsdt
{
  header: ACPISDTHeader,
  first_entry: PhysicalAddress,
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct Xsdp
{
  pub signature: [u8; 8],
  pub checksum: u8,
  pub oem_id: [u8; 6],
  pub revision: u8,
  pub rsdt_address: u32,
  pub length: u32,
  pub xsdt_address: u64,
  pub extended_checksum: u8,
  pub reserved: [u8; 3],
}

impl Xsdt
{
  pub fn new(p_xsdp: NonNull<Xsdp>) -> Option<NonNull<Self>>
  {
    unsafe {
      let xd_addr: PhysicalAddress = (*p_xsdp.as_ptr()).xsdt_address;
      CURRENT_PROC
        .lock()
        .as_mut()
        .unwrap()
        .ptm_operation(|ptm| {
          let old_temp = ptm.get_temp();
          let offset = old_temp % PAGE_SIZE;
          let ptr = (ptm.map_temp(get_containing_page(xd_addr))? + offset) as *mut Self;
          let length = (*ptr).header.length;
          ptm.map_temp(ptr as VirtualAddress)?;
          ptm.map_into(xd_addr, length as usize, PAGING_RW | PAGING_PRESENT)
        })
        .ok()
        .and_then(|x| NonNull::new(x as *mut Self))
    }
  }
  pub fn find_table<T: AcpiTable>(&self, signature: &str) -> Option<NonNull<T>>
  {
    let entry_count = (self.header.length as usize - size_of::<ACPISDTHeader>()) / 8;
    let entry_phys = addr_of!(self.first_entry) as *const PhysicalAddress;
    let sig_bytes = signature.as_bytes();
    CURRENT_PROC
      .lock()
      .as_mut()
      .unwrap()
      .ptm_operation(|ptm| {
        let old_temp = ptm.get_temp();
        let mut ret = Ok(0);
        for i in 0..entry_count
        {
          let entry_phy = unsafe { entry_phys.add(i).read() };
          let offset = entry_phy % PAGE_SIZE;
          let p_header =
            (ptm.map_temp(get_containing_page(entry_phy))? + offset) as *const ACPISDTHeader;
          let header = unsafe { *p_header };
          if header.signature.eq(sig_bytes)
          {
            if ACPISDTHeader::validate_checksum(p_header)
            {
              ret = ptm.map_into(
                entry_phy,
                header.length as usize,
                PAGING_RW | PAGING_PRESENT,
              );
              break;
            }
          }
        }
        ptm.map_temp(old_temp)?;
        ret
      })
      .ok()
      .and_then(|x| NonNull::new(x as *mut T))
  }
}

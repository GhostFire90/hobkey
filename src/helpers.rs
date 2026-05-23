use core::ops::{Deref, DerefMut};

use crate::memory::{
  paging::{PageTableManager, PtmError, VirtualAddress},
  pmm::PhysicalAddress,
  HHDM_OFFSET,
};

pub fn atou(bytes: &[u8], base: u8) -> Option<usize>
{
  if base > 16
  {
    None
  }
  else
  {
    let chars = "0123456789abcdef".as_bytes();

    Some(
      bytes
        .iter()
        .rev()
        .enumerate()
        .skip(1)
        .map(|(i, x)| {
          let mut ch = *x;
          ch.make_ascii_lowercase();
          let idx = chars.binary_search(&ch).unwrap();
          (base as usize).checked_pow(i.try_into().unwrap()).unwrap() * idx
        })
        .sum(),
    )
  }
}

pub fn map_phy_temp<T: DerefMut<Target = PageTableManager>>(
  phy_addr: PhysicalAddress,
  o_ptm: &mut Option<T>,
) -> Result<VirtualAddress, PtmError>
{
  if let Some(ptm) = o_ptm
  {
    ptm.map_temp(phy_addr)
  }
  else
  {
    let hhdm = HHDM_OFFSET.get().clone();
    Ok(phy_addr + hhdm)
  }
}

pub fn get_temp_addr<T: Deref<Target = PageTableManager>>(o_ptm: &Option<T>) -> PhysicalAddress
{
  if let Some(ptm) = o_ptm
  {
    ptm.get_temp()
  }
  else
  {
    0
  }
}

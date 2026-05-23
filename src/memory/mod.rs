use core::ops::Range;

use crate::{limine_req::HHDM_REQ, spinlock::LazySpinlock};

pub mod alloc;
pub mod mmap;
pub mod paging;
pub mod pmm;

pub const PAGE_SIZE: u64 = 4096;
pub const PS_USIZE: usize = PAGE_SIZE as usize;
pub static HHDM_OFFSET: LazySpinlock<u64> =
  LazySpinlock::new(|| HHDM_REQ.get_response().unwrap().offset());

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone)]
pub struct MemoryRange(pub Range<u64>);

impl<T: Into<u64>> From<Range<T>> for MemoryRange
{
  fn from(value: Range<T>) -> Self
  {
    Self(value.start.into()..value.end.into())
  }
}

impl<T: From<u64>> Into<Range<T>> for MemoryRange
{
  fn into(self) -> Range<T>
  {
    self.0.start.into()..self.0.end.into()
  }
}

impl PartialOrd for MemoryRange
{
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>
  {
    self.0.start.partial_cmp(&other.0.start)
  }
}

impl Ord for MemoryRange
{
  fn cmp(&self, other: &Self) -> core::cmp::Ordering
  {
    self.0.start.cmp(&other.0.start)
  }
}

impl MemoryRange
{
  pub fn len(&self) -> usize
  {
    (self.0.end - self.0.start) as usize
  }
  /// Returns: None if rhs.start != self.end
  pub fn append(&self, rhs: &Self) -> Option<Self>
  {
    if self.0.start != rhs.0.end
    {
      None
    }
    else
    {
      Some(Self(self.0.start..rhs.0.end))
    }
  }

  pub fn shrink(&self, length: usize) -> Option<(Self, Self)>
  {
    if self.len() >= length
    {
      let new_start = self.0.start + length as u64;
      Some((Self(self.0.start..new_start), Self(new_start..self.0.end)))
    }
    else
    {
      None
    }
  }
}

/// Gets the page that addr is inside of
pub fn get_containing_page<T: Into<u64> + From<u64>>(addr: T) -> T
{
  let addr = addr.into();
  if addr & (PAGE_SIZE - 1) == 0
  {
    return addr.into();
  }

  let mask = !(addr & (PAGE_SIZE - 1));
  (addr & mask).into()
}

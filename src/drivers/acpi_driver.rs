use core::ptr::NonNull;

use acpi::{Handler, PhysicalMapping};
use alloc::sync::Arc;

use crate::memory::{
  paging::{paging_flags, PageTableManager},
  pmm::PhysicalAddress,
  PS_USIZE,
};

struct HobkeyHandler;
#[derive(Clone)]
struct HandlerWrapper
{
  inner: Arc<HobkeyHandler>,
}

fn containing_page(address: usize) -> PhysicalAddress
{
  let remainder = address % PS_USIZE;
  (address - remainder) as PhysicalAddress
}

impl Handler for HandlerWrapper
{
  unsafe fn map_physical_region<T>(
    &self,
    physical_address: usize,
    size: usize,
  ) -> acpi::PhysicalMapping<Self, T>
  {
    let actual_phy = containing_page(physical_address);
    let actual_size = size.next_multiple_of(PS_USIZE) as u64;
    let virt = PageTableManager::extend_kernel_map_range(
      actual_phy,
      actual_size as usize,
      paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT,
    )
    .unwrap();

    PhysicalMapping {
      physical_start: physical_address,
      virtual_start: NonNull::new((virt as usize + (physical_address % PS_USIZE)) as *mut T)
        .unwrap(),
      region_length: size,
      mapped_length: actual_size as usize,
      handler: self.clone(),
    }
  }

  fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>)
  {
    todo!()
  }

  fn read_u8(&self, address: usize) -> u8
  {
    todo!()
  }

  fn read_u16(&self, address: usize) -> u16
  {
    todo!()
  }

  fn read_u32(&self, address: usize) -> u32
  {
    todo!()
  }

  fn read_u64(&self, address: usize) -> u64
  {
    todo!()
  }

  fn write_u8(&self, address: usize, value: u8)
  {
    todo!()
  }

  fn write_u16(&self, address: usize, value: u16)
  {
    todo!()
  }

  fn write_u32(&self, address: usize, value: u32)
  {
    todo!()
  }

  fn write_u64(&self, address: usize, value: u64)
  {
    todo!()
  }

  fn read_io_u8(&self, port: u16) -> u8
  {
    todo!()
  }

  fn read_io_u16(&self, port: u16) -> u16
  {
    todo!()
  }

  fn read_io_u32(&self, port: u16) -> u32
  {
    todo!()
  }

  fn write_io_u8(&self, port: u16, value: u8)
  {
    todo!()
  }

  fn write_io_u16(&self, port: u16, value: u16)
  {
    todo!()
  }

  fn write_io_u32(&self, port: u16, value: u32)
  {
    todo!()
  }

  fn read_pci_u8(&self, address: acpi::PciAddress, offset: u16) -> u8
  {
    todo!()
  }

  fn read_pci_u16(&self, address: acpi::PciAddress, offset: u16) -> u16
  {
    todo!()
  }

  fn read_pci_u32(&self, address: acpi::PciAddress, offset: u16) -> u32
  {
    todo!()
  }

  fn write_pci_u8(&self, address: acpi::PciAddress, offset: u16, value: u8)
  {
    todo!()
  }

  fn write_pci_u16(&self, address: acpi::PciAddress, offset: u16, value: u16)
  {
    todo!()
  }

  fn write_pci_u32(&self, address: acpi::PciAddress, offset: u16, value: u32)
  {
    todo!()
  }

  fn nanos_since_boot(&self) -> u64
  {
    todo!()
  }

  fn stall(&self, microseconds: u64)
  {
    todo!()
  }

  fn sleep(&self, milliseconds: u64)
  {
    todo!()
  }

  fn create_mutex(&self) -> acpi::Handle
  {
    todo!()
  }

  fn acquire(&self, mutex: acpi::Handle, timeout: u16) -> Result<(), acpi::aml::AmlError>
  {
    todo!()
  }

  fn release(&self, mutex: acpi::Handle)
  {
    todo!()
  }
}

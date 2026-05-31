use core::{marker::PhantomData, ptr::NonNull};

use crate::drivers::acpi::{ACPISDTHeader, AcpiTable};

const PIC1: usize = 0x20;
const PIC2: usize = 0xa0;

// Task priority regs

/// Task priority Register RW
const TPR_OFFSET: usize = 0x080;
/// Processor priority register RO
const PPR_OFFSET: usize = 0x0a;

/// Spurious interrupt vector R/W
const SIVR_OFFSET: usize = 0x0f0;
/// end of interrupt WO
const EOI_OFFSET: usize = 0x0b0;

// Timer regs

/// Initial count, uint32_t R/W
const ICNT_OFFSET: usize = 0x380;
/// Current count, uint32_t RO
const CCNT_OFFSET: usize = 0x390;
/// Divide config, only access bits 0,1,3 R/W
const DCNF_OFFSET: usize = 0x3e0;
/// LVT entry for timer, set this to the interrupt you want, uint32_t R/W
const TIVR_OFFSET: usize = 0x320;

#[repr(packed)]
pub struct MADT
{
  header: ACPISDTHeader,
  local_apic: u32,
  flags: u32,
}

impl AcpiTable for MADT {}

pub struct Apic<'a>
{
  address: NonNull<u8>,
  _boo: PhantomData<&'a u32>,
}

impl<'a> Apic<'a>
{
  pub fn get_sivr(&self) -> &'a mut u32
  {
    unsafe { self.address.byte_add(SIVR_OFFSET).cast().as_mut() }
  }
  pub fn get_tpr(&self) -> &'a mut u32
  {
    unsafe { self.address.byte_add(TPR_OFFSET).cast().as_mut() }
  }
  pub fn get_tivr(&self) -> &'a mut u32
  {
    unsafe { self.address.byte_add(TIVR_OFFSET).cast().as_mut() }
  }
  pub fn get_dcnf(&self) -> &'a mut u32
  {
    unsafe { self.address.byte_add(DCNF_OFFSET).cast().as_mut() }
  }

  pub fn get_icnt(&self) -> &'a mut u32
  {
    unsafe { self.address.byte_add(ICNT_OFFSET).cast().as_mut() }
  }
}

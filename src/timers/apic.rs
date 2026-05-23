use crate::drivers::acpi::{ACPISDTHeader, AcpiTable};

const PIC1: usize = 0x20;
const PIC2: usize = 0xA0;

const SIVR_OFFSET: usize = 0x0f0; // Spurious interrupt vector R/W
const EOI_OFFSET: usize = 0x0b0; // end of interrupt WO

// Timer regs
const ICNT_OFFSET: usize = 0x380; // Initial count, uint32_t R/W
const CCNT_OFFSET: usize = 0x390; // Current count, uint32_t RO
const DCNF_OFFSET: usize = 0x3e0; // Divide config, only access bits 0,1,3 R/W
const TIVR_OFFSET: usize = 0x320; // LVT entry for timer, set this to the interrupt you want, uint32_t R/W

#[repr(packed)]
pub struct MADT
{
  header: ACPISDTHeader,
  local_apic: u32,
  flags: u32,
}

impl AcpiTable for MADT {}

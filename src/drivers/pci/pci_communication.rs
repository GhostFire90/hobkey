use crate::{
  drivers::pci::{class_codes::ClassCodeRegister, PciType},
  utils::io_port_helper::{inl, outl},
};
use core::mem::transmute_copy;

const CONFIG_ADDR_PORT: u16 = 0xCF8;
const CONFIG_DATA_PORT: u16 = 0xCFC;

#[repr(packed)]
pub struct PciDevice
{
  bus: u8,
  device_function: u8,
}

impl PciDevice
{
  pub fn new(bus: u8, device: u8, function: u8) -> Self
  {
    Self {
      bus,
      device_function: (device << 3) | (function & 0x3),
    }
  }
  fn format_address(&self, enable: bool, offset: u8) -> u32
  {
    let mut addr = unsafe { transmute_copy::<Self, u16>(self) as u32 } << 8;
    addr |= (offset as u32) & 0xFC;
    addr |= if enable { 0x80000000 } else { 0x0 };
    addr
  }
  pub fn read_dword(&self, offset: u8) -> u32
  {
    outl(CONFIG_ADDR_PORT, self.format_address(true, offset));
    inl(CONFIG_DATA_PORT)
  }

  pub fn exists(&self) -> bool
  {
    let res = self.read_dword(0x0) & 0xFFFF;
    return res != 0xFFFF;
  }

  pub fn get_device_type(&self) -> PciType
  {
    let reg: ClassCodeRegister = self.read_dword(0x8).into();
    reg.parse()
  }
}

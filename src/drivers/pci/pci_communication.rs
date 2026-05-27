use crate::{
  drivers::pci::{class_codes::ClassCodeRegister, PciType},
  utils::io_port_helper::{inl, outl},
};
use core::mem::transmute_copy;

const CONFIG_ADDR_PORT: u16 = 0xCF8;
const CONFIG_DATA_PORT: u16 = 0xCFC;

pub const PCI_BUS_MAX: u8 = 255;
pub const PCI_DEVICE_MAX: u8 = 31;
pub const PCI_FUNCTION_MAX: u8 = 7;

#[derive(Debug, Clone, Copy)]
pub enum PciAddressError
{
  BusOutOfRange(u8),
  DeviceOutOfRange(u8),
  FunctionOutOfRange(u8),
  RegisterAlignment(u8),
}

#[repr(packed)]
pub struct PciDeviceAddress
{
  bus: u8,
  device: u8,
  function: u8,
}

impl PciDeviceAddress
{
  pub fn new(bus: u8, device: u8, function: u8) -> Result<Self, PciAddressError>
  {
    if device > PCI_DEVICE_MAX
    {
      Err(PciAddressError::DeviceOutOfRange(device))
    }
    else if bus > PCI_BUS_MAX
    {
      Err(PciAddressError::BusOutOfRange(bus))
    }
    else if function > PCI_FUNCTION_MAX
    {
      Err(PciAddressError::FunctionOutOfRange(function))
    }
    else
    {
      Ok(Self {
        bus,
        device,
        function,
      })
    }
  }
  fn format_address(&self, enable: bool, offset: u8) -> Result<u32, PciAddressError>
  {
    if offset & 0x3 > 0
    {
      return Err(PciAddressError::RegisterAlignment(offset));
    }
    let mut ret: u32 = if enable { 1 << 31 } else { 0 };
    ret |= (self.bus as u32) << 16;
    ret |= (self.device as u32) << 11;
    ret |= (self.function as u32) << 8;
    ret |= offset as u32;

    Ok(ret)
  }
  pub fn read_dword(&self, offset: u8) -> Result<u32, PciAddressError>
  {
    outl(CONFIG_ADDR_PORT, self.format_address(true, offset)?);
    Ok(inl(CONFIG_DATA_PORT))
  }

  pub fn exists(&self) -> bool
  {
    let res = self.read_dword(0x0).unwrap() & 0xFFFF;
    return res != 0xFFFF;
  }

  pub fn get_device_type(&self) -> PciType
  {
    let reg: ClassCodeRegister = self.read_dword(0x8).unwrap().into();
    reg.parse()
  }

  pub fn is_multi_function(&self) -> bool
  {
    (self.read_dword(0xC).unwrap() >> 16) & (1 << 7) > 0
  }
}

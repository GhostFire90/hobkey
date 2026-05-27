use crate::utils::io_port_helper;

const COUNT_RX_DESC: usize = 32;
const COUNT_TX_DESC: usize = 8;

#[repr(packed)]
struct RxDescriptor
{
  addr: u64,
  len: u16,
  checksum: u16,
  status: u8,
  errors: u8,
  special: u16,
}
#[repr(packed)]
struct TxDescriptor
{
  addr: u64,
  length: u16,
  cso: u8,
  cmd: u8,
  status: u8,
  css: u8,
  special: u16,
}

pub struct E1000
{
  io_base: u16,
  eeprom_exists: bool,
  mac_buffer: [u8; 6],
}

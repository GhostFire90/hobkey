/**
 * # class_codes.rs
 * Handles making rustlike stuff out of the PCI class stuff
 * ## TODOS
 * - ADD Program interface stuff in a sane way
 */

macro_rules! panic_unknown_subclass {
  ($name:literal, $subclass:ident) => {
    panic!("[PCI] Unknown {} subclass: {}", $name, $subclass)
  };
}
macro_rules! panic_unknown_interface {
  ($name:literal, $interface:ident) => {
    panic!("[PCI] Unknown {} interface: {}", $name, $interface)
  };
}

mod class_codes_u8
{
  pub const UNCLASSIFIED: u8 = 0;
  pub const MASS_STORAGE_CONTROLLER: u8 = 0x01;
  pub const NETWORK_CONTROLLER: u8 = 0x02;
  pub const DISPLAY_CONTROLLER: u8 = 0x03;
  pub const MUTLIMEDIA_CONTROLLER: u8 = 0x04;
  pub const MEMORY_CONTROLLER: u8 = 0x05;
  pub const BRIDGE: u8 = 0x06;
  pub const SIMPLE_COM_CONTROLLER: u8 = 0x07;
  pub const BASE_SYS_PERIPHERAL: u8 = 0x08;
  pub const INPUT_DEVICE_CONTROLLER: u8 = 0x09;
  pub const DOCKING_STATION: u8 = 0x0A;
  pub const PROCESSOR: u8 = 0x0b;
  pub const SERIAL_BUS_CONTROLLER: u8 = 0x0c;
  pub const WIRELESS_CONTROLLER: u8 = 0x0d;
  pub const INTELLIGENT_CONTROLLER: u8 = 0x0e;
  pub const SATELLITE_COM_CONTROLLER: u8 = 0x0f;
  pub const ENCRYPTION_CONTROLLER: u8 = 0x10;
  pub const SIGNAL_PROCESSING_CONTROLLER: u8 = 0x11;
  pub const PROCESSING_ACCELERATOR: u8 = 0x12;
  pub const NON_ESSENTIAL_INSTRUMENTATION: u8 = 0x13;
  pub const CO_PROCESSOR: u8 = 0x40;
  pub const UNASSIGNED: u8 = 0xff;
}

mod parsers
{
  use core::panic;

  use super::program_interfaces::*;
  use super::*;
  pub fn unclassified(subclass: u8) -> UnclassifiedType
  {
    match subclass
    {
      0x0 => UnclassifiedType::NonVGA,
      0x1 => UnclassifiedType::VGA,
      a => panic_unknown_subclass!("Unclassified", a),
    }
  }
  pub fn msc(subclass: u8, prog_if: u8) -> MassStorageType
  {
    match subclass
    {
      0x00 => MassStorageType::SCSIBus,
      0x01 =>
      {
        MassStorageType::IDE(match prog_if
        {
          0x00 => IDEInterface::ISACompatOnly,
          0x05 => IDEInterface::PCINativeOnly,
          0x0A => IDEInterface::ISACompatBothPCI,
          0x0F => IDEInterface::PCINativeBothISA,
          0x80 => IDEInterface::ISACompatOnlyBusMastering,
          0x85 => IDEInterface::PCINativeOnlyBusMastering,
          0x8A => IDEInterface::ISACompatBothPCIBusMastering,
          0x8F => IDEInterface::PCINativeBothISABusMastering,
          a => panic_unknown_interface!("IDE", a),
        })
      }
      0x02 => MassStorageType::FloppyDisk,
      0x03 => MassStorageType::IPIBus,
      0x04 => MassStorageType::RAID,
      0x05 =>
      {
        MassStorageType::ATA(match prog_if
        {
          0x20 => ATAInterface::SingleDMA,
          0x30 => ATAInterface::ChainedDMA,
          a => panic_unknown_interface!("ATA", a),
        })
      }
      0x06 =>
      {
        MassStorageType::SATA(match prog_if
        {
          0x00 => SATAInterface::VendorSpecific,
          0x01 => SATAInterface::AHCI,
          0x02 => SATAInterface::SerialStorage,
          a => panic_unknown_interface!("SATA", a),
        })
      }
      0x07 =>
      {
        MassStorageType::SerialSCSI(match prog_if
        {
          0x00 => SerialSCSIInterface::SAS,
          0x01 => SerialSCSIInterface::SerialStorage,
          a => panic_unknown_interface!("Serial SCSI", a),
        })
      }
      0x08 =>
      {
        MassStorageType::NonVolatileMemory(match prog_if
        {
          0x01 => NonVolatileMemoryInterface::NVMHCI,
          0x02 => NonVolatileMemoryInterface::NVME,
          a => panic_unknown_interface!("NVM", a),
        })
      }
      0x80 => MassStorageType::Other,
      a => panic_unknown_subclass!("Mass Storage Controller", a),
    }
  }
  pub fn networkcontroller(subclass: u8) -> NetworkType
  {
    match subclass
    {
      0x00 => NetworkType::Ethernet,
      0x01 => NetworkType::TokenRing,
      0x02 => NetworkType::FFDI,
      0x03 => NetworkType::ATM,
      0x04 => NetworkType::ISDN,
      0x05 => NetworkType::WorldFip,
      0x06 => NetworkType::PICMG,
      0x07 => NetworkType::Infiniband,
      0x08 => NetworkType::Fabric,
      0x80 => NetworkType::Other,
      a => panic_unknown_subclass!("Network Controller", a),
    }
  }
  pub fn displaycontroller(subclass: u8, prog_if: u8) -> DisplayType
  {
    match (subclass, prog_if)
    {
      (0x00, 0x00) => DisplayType::VGACompatible(VGACompatInteface::VGA),
      (0x00, 0x01) => DisplayType::VGACompatible(VGACompatInteface::Compat8514),
      (0x00, a) => panic_unknown_interface!("VGA Display", a),
      (0x01, _) => DisplayType::XGA,
      (0x02, _) => DisplayType::ThreeD,
      (0x80, _) => DisplayType::Other,
      (a, _) => panic_unknown_subclass!("Display", a),
    }
  }
  pub fn multimedia(subclass: u8) -> MultiMediaType
  {
    match subclass
    {
      0x00 => MultiMediaType::Video,
      0x01 => MultiMediaType::AudioController,
      0x02 => MultiMediaType::ComputerTelephony,
      0x03 => MultiMediaType::AudioDevice,
      0x80 => MultiMediaType::Other,
      a => panic_unknown_subclass!("MultiMedia", a),
    }
  }
  pub fn memory(subclass: u8) -> MemoryControllerType
  {
    match subclass
    {
      0x00 => MemoryControllerType::RAM,
      0x01 => MemoryControllerType::Flash,
      0x80 => MemoryControllerType::Other,
      a => panic_unknown_subclass!("Memory Controller", a),
    }
  }
  pub fn bridge(subclass: u8, _prog_if: u8) -> BridgeType
  {
    match subclass
    {
      0x00 => BridgeType::Host,
      0x01 => BridgeType::ISA,
      0x02 => BridgeType::EISA,
      0x03 => BridgeType::MCA,
      0x04 => BridgeType::PciToPci,
      0x05 => BridgeType::PCMCIA,
      0x06 => BridgeType::NuBus,
      0x07 => BridgeType::CardBus,
      0x08 => BridgeType::RACEway,
      0x09 => BridgeType::PciToPci,
      0x0a => BridgeType::InfibandPciBridge,
      0x80 => BridgeType::Other,
      a => panic_unknown_subclass!("PCI Bridge", a),
    }
  }
  pub fn simplecom(subclass: u8) -> SimpleComType
  {
    match subclass
    {
      0x00 => SimpleComType::Serial,
      0x01 => SimpleComType::Parallel,
      0x02 => SimpleComType::MultiportSerial,
      0x03 => SimpleComType::Modem,
      0x04 => SimpleComType::GPIB,
      0x05 => SimpleComType::SmartCard,
      0x80 => SimpleComType::Other,
      a => panic_unknown_subclass!("Simple Com controller", a),
    }
  }
  pub fn baseperipheral(subclass: u8) -> BasePeripheralType
  {
    match subclass
    {
      0x00 => BasePeripheralType::PIC,
      0x01 => BasePeripheralType::DMA,
      0x02 => BasePeripheralType::Timer,
      0x03 => BasePeripheralType::RTC,
      0x04 => BasePeripheralType::PCIHotPlug,
      0x05 => BasePeripheralType::SDHost,
      0x06 => BasePeripheralType::IOMMU,
      0x80 => BasePeripheralType::Other,
      a => panic_unknown_subclass!("Base Sys Peripheral", a),
    }
  }
  pub fn inputdevice(subclass: u8) -> InputDeviceType
  {
    match subclass
    {
      0x00 => InputDeviceType::Keyboard,
      0x01 => InputDeviceType::DigitizerPen,
      0x02 => InputDeviceType::Mouse,
      0x03 => InputDeviceType::Scanner,
      0x04 => InputDeviceType::GamePort,
      0x80 => InputDeviceType::Other,
      a => panic_unknown_subclass!("Input Device", a),
    }
  }
  pub fn dockingstation(subclass: u8) -> DockingStationType
  {
    match subclass
    {
      0x00 => DockingStationType::Generic,
      0x80 => DockingStationType::Other,
      a => panic_unknown_subclass!("Docking Station", a),
    }
  }
  pub fn processor(subclass: u8) -> ProcessorType
  {
    match subclass
    {
      0x00 => ProcessorType::P386,
      0x01 => ProcessorType::P486,
      0x02 => ProcessorType::Pentium,
      0x03 => ProcessorType::PentiumPro,
      0x10 => ProcessorType::Alpha,
      0x20 => ProcessorType::PowerPC,
      0x30 => ProcessorType::MIPS,
      0x40 => ProcessorType::CoProcessor,
      0x80 => ProcessorType::Other,
      a => panic_unknown_subclass!("Processor", a),
    }
  }
  pub fn serialbus(subclass: u8, prog_if: u8) -> SerialBusType
  {
    match (subclass, prog_if)
    {
      (0x00, _) => SerialBusType::FireWire,
      (0x01, _) => SerialBusType::ACCESSBus,
      (0x02, _) => SerialBusType::SSA,
      (0x03, 0x00) => SerialBusType::USB(USBInterface::UHCI),
      (0x03, 0x10) => SerialBusType::USB(USBInterface::OHCI),
      (0x03, 0x20) => SerialBusType::USB(USBInterface::EHCI),
      (0x03, 0x30) => SerialBusType::USB(USBInterface::XHCI),
      (0x03, 0x80) => SerialBusType::USB(USBInterface::Unspecified),
      (0x03, 0xFE) => SerialBusType::USB(USBInterface::USBDevice),
      (0x03, a) => panic_unknown_interface!("USB", a),
      (0x04, _) => SerialBusType::FibreChannel,
      (0x05, _) => SerialBusType::SMBus,
      (0x06, _) => SerialBusType::InfiniBand,
      (0x07, _) => SerialBusType::IPMIInterface,
      (0x08, _) => SerialBusType::SERCOSInterface,
      (0x09, _) => SerialBusType::CANbus,
      (0x80, _) => SerialBusType::Other,
      (a, _) => panic_unknown_subclass!("USB", a),
    }
  }
  pub fn wirelesscontroller(subclass: u8) -> WirelessType
  {
    match subclass
    {
      0x00 => WirelessType::iRDACompatible,
      0x01 => WirelessType::ConsumerIR,
      0x10 => WirelessType::RF,
      0x11 => WirelessType::Bluetooth,
      0x12 => WirelessType::Broadband,
      0x20 => WirelessType::Ethernet_802_1a,
      0x21 => WirelessType::Ethernet_802_1b,
      0x80 => WirelessType::Other,
      a => panic_unknown_subclass!("Wireless Controller", a),
    }
  }
  pub fn satelitecom(subclass: u8) -> SatelliteComType
  {
    match subclass
    {
      0x01 => SatelliteComType::TV,
      0x02 => SatelliteComType::Audio,
      0x03 => SatelliteComType::Voice,
      0x04 => SatelliteComType::Data,
      a => panic_unknown_subclass!("Satellite Communication", a),
    }
  }
  pub fn encryptioncontroller(subclass: u8) -> EncryptionType
  {
    match subclass
    {
      0x00 => EncryptionType::NetCodec,
      0x10 => EncryptionType::EntertainmentCodec,
      0x80 => EncryptionType::Other,
      a => panic_unknown_subclass!("Encryption Controller", a),
    }
  }
  pub fn signalprocessingcontroller(subclass: u8) -> SignalProcessingType
  {
    match subclass
    {
      0x00 => SignalProcessingType::DPIOModules,
      0x01 => SignalProcessingType::PerformanceCounters,
      0x10 => SignalProcessingType::CommSynchronizer,
      0x20 => SignalProcessingType::SignalProcessingManagement,
      0x80 => SignalProcessingType::Other,
      a => panic_unknown_subclass!("Signal Processing Controller", a),
    }
  }
}

pub struct ClassCodeRegister
{
  pub class_code: u8,
  pub subclass_code: u8,
  pub prog_if: u8,
  pub _revision_id: u8,
}
impl From<u32> for ClassCodeRegister
{
  fn from(value: u32) -> Self
  {
    Self {
      class_code: (value >> 24) as u8,
      subclass_code: (value >> 16) as u8,
      prog_if: (value >> 8) as u8,
      _revision_id: value as u8,
    }
  }
}

impl ClassCodeRegister
{
  pub fn parse(self) -> PciType
  {
    use class_codes_u8::*;
    use parsers::*;
    match self.class_code
    {
      UNCLASSIFIED => PciType::Unclassified(unclassified(self.subclass_code)),
      MASS_STORAGE_CONTROLLER =>
      {
        PciType::MassStorageController(msc(self.subclass_code, self.prog_if))
      }
      NETWORK_CONTROLLER => PciType::NetworkController(networkcontroller(self.subclass_code)),
      DISPLAY_CONTROLLER =>
      {
        PciType::DisplayController(displaycontroller(self.subclass_code, self.prog_if))
      }
      MUTLIMEDIA_CONTROLLER => PciType::MutlimediaController(multimedia(self.subclass_code)),
      MEMORY_CONTROLLER => PciType::MemoryController(memory(self.subclass_code)),
      BRIDGE => PciType::Bridge(bridge(self.subclass_code, self.prog_if)),
      SIMPLE_COM_CONTROLLER => PciType::SimpleComController(simplecom(self.subclass_code)),
      BASE_SYS_PERIPHERAL => PciType::BaseSysPeripheral(baseperipheral(self.subclass_code)),
      INPUT_DEVICE_CONTROLLER => PciType::InputDeviceController(inputdevice(self.subclass_code)),
      DOCKING_STATION => PciType::DockingStation(dockingstation(self.subclass_code)),
      PROCESSOR => PciType::Processor(processor(self.subclass_code)),
      SERIAL_BUS_CONTROLLER =>
      {
        PciType::SerialBusController(serialbus(self.subclass_code, self.prog_if))
      }
      WIRELESS_CONTROLLER => PciType::WirelessController(wirelesscontroller(self.subclass_code)),
      INTELLIGENT_CONTROLLER => PciType::IntelligentController,
      SATELLITE_COM_CONTROLLER => PciType::SatelliteComController(satelitecom(self.subclass_code)),
      ENCRYPTION_CONTROLLER =>
      {
        PciType::EncryptionController(encryptioncontroller(self.subclass_code))
      }
      SIGNAL_PROCESSING_CONTROLLER =>
      {
        PciType::SignalProcessingController(signalprocessingcontroller(self.subclass_code))
      }
      PROCESSING_ACCELERATOR => PciType::ProcessingAccelerator,
      NON_ESSENTIAL_INSTRUMENTATION => PciType::NonEssentialInstrumentation,
      CO_PROCESSOR => PciType::CoProcessor,
      UNASSIGNED => PciType::Unassigned,
      a => panic!("Unknown class Code {}", a),
    }
  }
}

pub mod program_interfaces
{
  #[derive(Clone, Debug)]
  pub enum IDEInterface
  {
    ISACompatOnly,
    PCINativeOnly,
    ISACompatBothPCI,
    PCINativeBothISA,
    ISACompatOnlyBusMastering,
    PCINativeOnlyBusMastering,
    ISACompatBothPCIBusMastering,
    PCINativeBothISABusMastering,
  }
  #[derive(Clone, Debug)]
  pub enum ATAInterface
  {
    SingleDMA,
    ChainedDMA,
  }

  #[derive(Clone, Debug)]
  pub enum SATAInterface
  {
    VendorSpecific,
    AHCI,
    SerialStorage,
  }

  #[derive(Clone, Debug)]
  pub enum SerialSCSIInterface
  {
    SAS,
    SerialStorage,
  }
  #[derive(Clone, Debug)]
  pub enum NonVolatileMemoryInterface
  {
    NVMHCI,
    NVME,
  }

  #[derive(Clone, Debug)]
  pub enum VGACompatInteface
  {
    VGA,
    Compat8514,
  }

  #[derive(Clone, Debug)]
  pub enum USBInterface
  {
    UHCI,
    OHCI,
    EHCI,
    XHCI,
    Unspecified,
    USBDevice,
  }
}
use program_interfaces::*;

#[derive(Clone, Debug)]
pub enum UnclassifiedType
{
  NonVGA,
  VGA,
}

#[derive(Clone, Debug)]
pub enum MassStorageType
{
  SCSIBus,
  IDE(IDEInterface),
  FloppyDisk,
  IPIBus,
  RAID,
  ATA(ATAInterface),
  SATA(SATAInterface),
  SerialSCSI(SerialSCSIInterface),
  NonVolatileMemory(NonVolatileMemoryInterface),
  Other,
}

#[derive(Clone, Debug)]
pub enum NetworkType
{
  Ethernet,
  TokenRing,
  FFDI,
  ATM,
  ISDN,
  WorldFip,
  PICMG,
  Infiniband,
  Fabric,
  Other,
}

#[derive(Clone, Debug)]
pub enum DisplayType
{
  VGACompatible(VGACompatInteface),
  XGA,
  ThreeD,
  Other,
}

#[derive(Clone, Debug)]
pub enum MultiMediaType
{
  Video,
  AudioController,
  ComputerTelephony,
  AudioDevice,
  Other,
}

#[derive(Clone, Debug)]
pub enum MemoryControllerType
{
  RAM,
  Flash,
  Other,
}

// !TODO: Gotta add whatever the difference between subclass 4 and 9 is
#[derive(Clone, Debug)]
pub enum BridgeType
{
  Host,
  ISA,
  EISA,
  MCA,
  PciToPci,
  PCMCIA,
  NuBus,
  CardBus,
  RACEway,
  InfibandPciBridge,
  Other,
}

#[derive(Clone, Debug)]
pub enum SimpleComType
{
  Serial,
  Parallel,
  MultiportSerial,
  Modem,
  GPIB,
  SmartCard,
  Other,
}

#[derive(Clone, Debug)]
pub enum BasePeripheralType
{
  PIC,
  DMA,
  Timer,
  RTC,
  PCIHotPlug,
  SDHost,
  IOMMU,
  Other,
}

#[derive(Clone, Debug)]
pub enum InputDeviceType
{
  Keyboard,
  DigitizerPen,
  Mouse,
  Scanner,
  GamePort,
  Other,
}
#[derive(Clone, Debug)]
pub enum DockingStationType
{
  Generic,
  Other,
}

#[derive(Clone, Debug)]
pub enum ProcessorType
{
  P386,
  P486,
  Pentium,
  PentiumPro,
  Alpha,
  PowerPC,
  MIPS,
  CoProcessor,
  Other,
}

#[derive(Clone, Debug)]
pub enum SerialBusType
{
  FireWire,
  ACCESSBus,
  SSA,
  USB(USBInterface),
  FibreChannel,
  SMBus,
  InfiniBand,
  IPMIInterface,
  SERCOSInterface,
  CANbus,
  Other,
}

#[derive(Clone, Debug)]
pub enum WirelessType
{
  iRDACompatible,
  ConsumerIR,
  RF,
  Bluetooth,
  Broadband,
  Ethernet_802_1a,
  Ethernet_802_1b,
  Other,
}

#[derive(Clone, Debug)]
pub enum SatelliteComType
{
  TV,
  Audio,
  Voice,
  Data,
}

#[derive(Clone, Debug)]
pub enum EncryptionType
{
  NetCodec,
  EntertainmentCodec,
  Other,
}

#[derive(Clone, Debug)]
pub enum SignalProcessingType
{
  DPIOModules,
  PerformanceCounters,
  CommSynchronizer,
  SignalProcessingManagement,
  Other,
}

#[derive(Clone, Debug)]
pub enum PciType
{
  Unclassified(UnclassifiedType),
  MassStorageController(MassStorageType),
  NetworkController(NetworkType),
  DisplayController(DisplayType),
  MutlimediaController(MultiMediaType),
  MemoryController(MemoryControllerType),
  Bridge(BridgeType),
  SimpleComController(SimpleComType),
  BaseSysPeripheral(BasePeripheralType),
  InputDeviceController(InputDeviceType),
  DockingStation(DockingStationType),
  Processor(ProcessorType),
  SerialBusController(SerialBusType),
  WirelessController(WirelessType),
  IntelligentController,
  SatelliteComController(SatelliteComType),
  EncryptionController(EncryptionType),
  SignalProcessingController(SignalProcessingType),
  ProcessingAccelerator,
  NonEssentialInstrumentation,
  CoProcessor,
  Unassigned,
}

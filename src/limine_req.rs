#![allow(dead_code)]

use limine::modules::InternalModule;
use limine::request::{
  ExecutableAddressRequest, FramebufferRequest, HhdmRequest, MemoryMapRequest, ModuleRequest,
  RsdpRequest,
};
use limine::BaseRevision;

pub static BASE_REVISION: BaseRevision = BaseRevision::with_revision(4);

pub static FB_REQ: FramebufferRequest = FramebufferRequest::new();
pub static MM_REQ: MemoryMapRequest = MemoryMapRequest::new();
pub static HHDM_REQ: HhdmRequest = HhdmRequest::new();
pub static KERNEL_REQ: ExecutableAddressRequest = ExecutableAddressRequest::new();
pub static MODULE_REQ: ModuleRequest =
  ModuleRequest::new().with_internal_modules(&[&InternalModule::new().with_path(c"initrd.tar")]);
pub static RSDP_REQ: RsdpRequest = RsdpRequest::new();

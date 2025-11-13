#![allow(dead_code)]


use limine::modules::InternalModule;
use limine::BaseRevision;
use limine::request::{
    FramebufferRequest,
    HhdmRequest,
    ExecutableAddressRequest,
    MemoryMapRequest,
    ModuleRequest
};

pub static BASE_REVISION : BaseRevision = BaseRevision::new();

pub static FB_REQ : FramebufferRequest = FramebufferRequest::new();
pub static MM_REQ : MemoryMapRequest = MemoryMapRequest::new();
pub static HHDM_REQ : HhdmRequest = HhdmRequest::new();
pub static KERNEL_REQ : ExecutableAddressRequest = ExecutableAddressRequest::new();
pub static MODULE_REQ : ModuleRequest = ModuleRequest::new().with_internal_modules(&[&InternalModule::new().with_path(c"initrd.tar")]);
 

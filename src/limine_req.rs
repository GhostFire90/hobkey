#![allow(dead_code)]


use limine::BaseRevision;
use limine::request::{FramebufferRequest, MemoryMapRequest, HhdmRequest};

pub static BASE_REVISION : BaseRevision = BaseRevision::new();

pub static FB_REQ : FramebufferRequest = FramebufferRequest::new();
pub static MM_REQ : MemoryMapRequest = MemoryMapRequest::new();
pub static HHDM_REQ : HhdmRequest = HhdmRequest::new();
 

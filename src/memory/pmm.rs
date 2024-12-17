use core::ptr;

use limine::memory_map::EntryType;

use super::paging::PageTableManager;
use super::PAGE_SIZE;
use crate::limine_req::{HHDM_REQ, MM_REQ};
use crate::spinlock::*;
use lazy_static::lazy_static;
use hobkey_rs_proc::preserve_temp_map;

struct PmmNode {
    next: u64,
    length: u64,
}
pub struct PMM {
    head: u64,
    available_mem : u64
}

#[derive(Debug)]
pub enum PmmError{
  OutOfMemory
}

pub type PhysicalAddress = u64;

lazy_static! {
    static ref PHYSICAL_MEMORY_MANAGER: Spinlock<PMM> = Spinlock::new(PMM::init());
}

impl PMM {
    pub(self) fn init() -> Self {
      let mut avail = 0;
      let memmap = MM_REQ.get_response().unwrap();
      let hhdm_offset = HHDM_REQ.get_response().unwrap().offset();
      let mut current: *mut PmmNode = ptr::null_mut();
      let mut last: *mut PmmNode = current;
      for entry in memmap.entries() {
        if entry.entry_type == EntryType::USABLE {
          avail += PAGE_SIZE;
          last = current;
          current = (entry.base + hhdm_offset) as *mut PmmNode;
          unsafe {
              current.write(PmmNode {
                  next: 0,
                  length: entry.length,
              });
              if !last.is_null() {
                  let mut l = last.read();
                  l.next = current as u64 - hhdm_offset;
                  last.write(l);
              }
          }
        }
      }
      if !current.is_null() && !last.is_null() {
        let mut e = unsafe { current.read() };
        e.next = last as u64 - hhdm_offset;
        unsafe {
            current.write(e);
        }
      }

      PMM {
          head: current as u64 - hhdm_offset,
          available_mem: avail
      }
    }

    #[preserve_temp_map]
    pub fn pop_page() -> Result<PhysicalAddress, PmmError> {
        let mut guard = PHYSICAL_MEMORY_MANAGER.lock();
        let pmm = guard.get_mut();
        if pmm.available_mem == 0{
          return Err(PmmError::OutOfMemory);
        }
        let head_entry = PageTableManager::map_temp(pmm.head) as *mut PmmNode;
        let mut head = unsafe { head_entry.read() };
        head.length -= PAGE_SIZE;
        let ret = pmm.head;
        unsafe {
            if head.length != 0 {
                pmm.head += PAGE_SIZE;
                head_entry.write(head);
            } else {
                pmm.head = head_entry.read().next;
            }
        };
        pmm.available_mem -= PAGE_SIZE;
        return Ok(ret)
    }
    #[preserve_temp_map]
    pub fn push_page(page: PhysicalAddress) {
      let mut guard = PHYSICAL_MEMORY_MANAGER.lock();
      let pmm = guard.get_mut();
      let old_head = unsafe {(PageTableManager::map_temp(pmm.head) as *mut PmmNode).read()};
      if page + PAGE_SIZE == pmm.head {
        let mapped = PageTableManager::map_temp(page) as *mut PmmNode;
        unsafe {mapped.write(PmmNode{next: old_head.next, length: old_head.length+PAGE_SIZE});};
        pmm.head = page;
      }
      else if pmm.head + old_head.length == page{
        let mapped = PageTableManager::map_temp(pmm.head) as *mut PmmNode;
        unsafe {mapped.write(PmmNode{next:old_head.next, length:old_head.length+PAGE_SIZE});};
      }
      else{
        let new_head =  PageTableManager::map_temp(page) as *mut PmmNode;
        unsafe {
          new_head.write(PmmNode{next : pmm.head, length: PAGE_SIZE});  
        };
        pmm.head = page;
        
      }
      pmm.available_mem += PAGE_SIZE;
    }
    
    pub fn get_avaiable_memory() -> u64{
      PHYSICAL_MEMORY_MANAGER.lock().get().available_mem
    }
  }

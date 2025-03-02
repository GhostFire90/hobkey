use core::ptr;

use limine::memory_map::EntryType;

use super::paging::PageTableManager;
use super::PAGE_SIZE;
use crate::limine_req::{HHDM_REQ, MM_REQ};
use crate::spinlock::*;
use hobkey_rs_proc::preserve_temp_map;


struct PmmNode {
    next: u64,
    length: u64,
}
pub struct PMM {
    head: u64,
    available_mem : u64,
    reclaimed : bool
}

#[derive(Debug)]
pub enum PmmError{
  NonReclaimable
}

pub type PhysicalAddress = u64;


static PHYSICAL_MEMORY_MANAGER: Spinlock<PMM> = Spinlock::new(PMM::new());

impl PMM {
    pub(self) const fn new() -> Self{
      Self { head: 0, available_mem: 0, reclaimed: false }
    }


    pub fn init(){
      let mut avail = 0;
      let memmap = MM_REQ.get_response().unwrap();
      let hhdm_offset = HHDM_REQ.get_response().unwrap().offset();
      let mut current: *mut PmmNode = ptr::null_mut();
      let mut last: *mut PmmNode = current;
      for entry in memmap.entries().iter().filter(|x|x.entry_type == EntryType::USABLE) {
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
      if !current.is_null() && !last.is_null() {
        let mut e = unsafe { current.read() };
        e.next = last as u64 - hhdm_offset;
        unsafe {
            current.write(e);
        }
      }

      *PHYSICAL_MEMORY_MANAGER.lock().get_mut() = 
      PMM {
          head: current as u64 - hhdm_offset,
          available_mem: avail,
          reclaimed: Default::default()
      };
    }

    pub fn reclaim_bootloader() -> Result<(), PmmError>{
      let mut guard = PHYSICAL_MEMORY_MANAGER.lock();
      let pmm = guard.get_mut();
      if pmm.reclaimed{
        return Err(PmmError::NonReclaimable);
      }
      pmm.reclaimed = true;
      drop(guard);

      let memmap = MM_REQ.get_response().unwrap();

      for node in memmap.entries().iter().filter(|x|x.entry_type == EntryType::BOOTLOADER_RECLAIMABLE){
        for pg_offset in (0..node.length).step_by(PAGE_SIZE as usize){
          PMM::push_page(node.base+pg_offset);
        }        
      }
      Ok(())
    }

    #[preserve_temp_map]
    pub extern "sysv64" fn pop_page() -> PhysicalAddress {
      let mut guard = PHYSICAL_MEMORY_MANAGER.lock();
      let pmm = guard.get_mut();
        
      if pmm.available_mem == 0{
        return 0;
      }
      let mut head_entry = PageTableManager::map_temp(pmm.head).unwrap() as *mut PmmNode;
      let mut head = unsafe { head_entry.read() };
      head.length -= PAGE_SIZE;
      let ret = pmm.head;
      unsafe {
          if head.length != 0 {
              pmm.head += PAGE_SIZE;
              head_entry = PageTableManager::map_temp(pmm.head).unwrap() as *mut PmmNode;
              head_entry.write(head);
          } 
          else {
              pmm.head = head_entry.read().next;
          }
          (PageTableManager::map_temp(ret).unwrap() as *mut u8).write_bytes(0, PAGE_SIZE as usize);
      };
      pmm.available_mem -= PAGE_SIZE;
      
      return ret
    }
    #[preserve_temp_map]
    pub extern "sysv64" fn push_page(page: PhysicalAddress) {
      let mut guard = PHYSICAL_MEMORY_MANAGER.lock();
      let pmm = guard.get_mut();


      let old_head = unsafe {(PageTableManager::map_temp(pmm.head).unwrap() as *mut PmmNode).read()};
      if page + PAGE_SIZE == pmm.head {
        let mapped = PageTableManager::map_temp(page).unwrap() as *mut PmmNode;
        unsafe {mapped.write(PmmNode{next: old_head.next, length: old_head.length+PAGE_SIZE});};
        pmm.head = page;
      }
      else if pmm.head + old_head.length == page{
        let mapped = PageTableManager::map_temp(pmm.head).unwrap() as *mut PmmNode;
        unsafe {mapped.write(PmmNode{next:old_head.next, length:old_head.length+PAGE_SIZE});};
      }
      else{
        let new_head =  PageTableManager::map_temp(page).unwrap() as *mut PmmNode;
        unsafe {
          new_head.write(PmmNode{next : pmm.head, length: PAGE_SIZE});  
        };
        pmm.head = page;
        
      }
      pmm.available_mem += PAGE_SIZE;
    }
    
    

    #[allow(dead_code)]
    pub fn get_avaiable_memory() -> u64{
      PHYSICAL_MEMORY_MANAGER.lock().get().available_mem
    }
  }

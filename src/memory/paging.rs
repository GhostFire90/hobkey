use hobkey_rs_proc::preserve_temp_map;
use lazy_static::lazy_static;

use crate::spinlock::*;
use super::pmm::{PhysicalAddress, PMM};
use crate::limine_req::HHDM_REQ;
use core::arch::global_asm;
use core::include_str;
use core::ops::{Index, IndexMut};


const TABLE_ENTRY_COUNT : usize = 512;

extern "C"{
    static MAX_PHY_BIT : u8;
    static MAX_VRT_BIT : u8;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct TableEntry(u64);
#[repr(transparent)]
struct MemoryTable{
    pub entries : [TableEntry; TABLE_ENTRY_COUNT]
}
pub enum TableLayer{
    PML4,
    PDPT,
    PDT,
    PT
}

pub mod paging_flags{
    pub const PAGING_PRESENT  : u64  = 1 << 0;  // Present; must be 1 to reference a paging table
    pub const PAGING_RW       : u64  = 1 << 1;  // Read/write; if 0, writes may not be allowed (see Section 4.6)
    pub const PAGING_USER     : u64  = 1 << 2;  // User/supervisor; if 0, user-mode accesses are not allowed (see Section 4.6)
    pub const PAGING_PWT      : u64  = 1 << 3;  // Page-level write-through; indirectly determines memory type (see Section 4.9.2)
    pub const PAGING_PCD      : u64  = 1 << 4;  // Page-level cache disable; indirectly determines memory type (see Section 4.9.2)
    pub const PAGING_ACCESSED : u64  = 1 << 5;  // Accessed; indicates whether this entry has been used (see Section 4.8)
    pub const PAGING_R        : u64  = 1 << 11; // For ordinary paging, ignored; for HLAT paging, restart (see Section 4.8)
}

pub struct PageTableManager{
    phy_addr : PhysicalAddress,
    tmp_address : VirtualAddress,
    ready : bool
}
type VirtualAddress = u64;

lazy_static!{
    //Todo fix this cause each "thread" will most likely have their own PTM
    static ref PAGE_TABLE_MANAGER : Spinlock<PageTableManager> = Spinlock::new(PageTableManager::new());

}

pub enum PtmError {
    NoMapping(VirtualAddress)
}

fn split_virtual(virt : VirtualAddress) -> ([u16; 4], u16){
    ([
        (virt >> 39) as u16 & 0x1ff,
        (virt >> 30) as u16 & 0x1ff,
        (virt >> 31) as u16 & 0x1ff,
        (virt >> 12) as u16 & 0x1ff,

    ], virt as u16 & 0xeff)
}

impl PageTableManager{
    pub(self) fn new() -> Self{
        PageTableManager {phy_addr : 0, tmp_address : 0, ready : false}
    }

    pub fn map_temp(phy_addr : PhysicalAddress) -> VirtualAddress{
        if !PAGE_TABLE_MANAGER.lock().get().ready{
            let offset = HHDM_REQ.get_response().unwrap().offset();
            phy_addr + offset
        }
        else {
            todo!()
        }
    }
    pub fn get_physical(virt_addr : VirtualAddress) -> Option<PhysicalAddress>{
        if !PAGE_TABLE_MANAGER.lock().get().ready{
            let offset = HHDM_REQ.get_response().unwrap().offset();
            Some(virt_addr-offset)
        }
        else{
            todo!()
        }
    }
    pub fn get_temp() -> PhysicalAddress{
        if !PAGE_TABLE_MANAGER.lock().get().ready{
            return 0;
        }
        else {
            todo!()
        }
    }
}
impl TableEntry {
    pub fn new(phy : PhysicalAddress, rw : bool) -> TableEntry{
        let phy_mask = unsafe {(1<<MAX_PHY_BIT) ^ (1<<12)};
        let mut val = 0;
        val |= (rw as u64) << 1;
        val |= (phy << 12) & phy_mask;
        TableEntry(val)
    }
    pub fn get_pointer(&self) -> PhysicalAddress{
        (self.0 >> 12) & unsafe{(1<<MAX_PHY_BIT)-1}
    }
    
    pub fn set_pointer(&mut self, ptr : PhysicalAddress){
        let phy_mask = unsafe {(1<<MAX_PHY_BIT) ^ (1<<12)};
        self.0 &= !phy_mask;
        self.0 |= (ptr << 12) & phy_mask;
    }
    pub fn set_flags(&mut self, flags : u64){
        self.0 &= !flags;
        self.0 |= flags;
    }
}
impl MemoryTable{
    #[preserve_temp_map]
    pub fn crawl(&self, virt : VirtualAddress, layer : TableLayer) -> Result<*const TableEntry, PtmError>{
        let (indexes, _) = split_virtual(virt);
        let guard = PAGE_TABLE_MANAGER.lock();
        let pmm = guard.get();
        let mut ret = PageTableManager::map_temp(pmm.phy_addr) as *const MemoryTable;
        let l : usize = layer.into(); 

        for i in 0..l{
            let current = unsafe {ret.read()};
            let phy = current[i].get_pointer();
            if phy == 0{
                return Err(PtmError::NoMapping(virt));
            }
            ret = PageTableManager::map_temp(phy) as *const MemoryTable; 
        }
        return Ok(unsafe{ret.cast::<TableEntry>().add(indexes[l].into())});
    }
    #[preserve_temp_map]
    pub fn crawl_alloc(&mut self, virt : VirtualAddress, table_layer : TableLayer, flags : u64) -> *mut TableEntry{
        let (indexes, _) = split_virtual(virt);
        let guard = PAGE_TABLE_MANAGER.lock();
        let pmm = guard.get();
        let mut ret = PageTableManager::map_temp(pmm.phy_addr) as *mut MemoryTable;
        let l : usize = table_layer.into(); 

        for i in 0..l{
            
            let mut entry = unsafe{ret.cast::<TableEntry>().add(i).read()};
            entry.check_and_alloc(flags);
            unsafe {ret.cast::<TableEntry>().add(i).write(entry)}
            ret = PageTableManager::map_temp(entry.get_pointer()) as *mut MemoryTable; 
        }
        return unsafe{ret.cast::<TableEntry>().add(indexes[l].into())};
    }
}


impl TableEntry{
    pub fn check_and_alloc(&mut self, flags : u64){
        let present = (self.0 & paging_flags::PAGING_PRESENT) != 0;
        if present{
            self.set_flags(flags);
        }
        else{
            let page = PMM::pop_page().unwrap();
            self.set_pointer(page);
            self.set_flags(flags);
        }
    }
}
impl Into<u64> for TableEntry {
    fn into(self) -> u64 {
        self.0
    }
}
impl From<u64> for TableEntry{
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<usize> for TableLayer{
    fn into(self) -> usize {
        match self {
            TableLayer::PML4 => 0,
            TableLayer::PDPT => 1,
            TableLayer::PDT => 2,
            TableLayer::PT => 2,
        }
    }
}

impl Index<usize> for MemoryTable{
    type Output = TableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}
impl IndexMut<usize> for MemoryTable{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}
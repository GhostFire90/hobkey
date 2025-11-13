#![allow(dead_code)]


use crate::drivers::serial::Serial;
use crate::{limine_req, spinlock::*};
use super::pmm::{PhysicalAddress, PmmError, PMM};
use super::PAGE_SIZE;
use crate::limine_req::{HHDM_REQ, KERNEL_REQ};
use core::arch::{global_asm, asm};
use core::{fmt, include_str};
use core::ops::{RangeInclusive, Shl};

use limine::request::StackSizeRequest;
static STACK_SIZE_REQ : StackSizeRequest = StackSizeRequest::new().with_size(PAGE_SIZE);

global_asm!(include_str!("paging.s"));


extern "sysv64"{
    fn invalidate_page(pg : u64);
}

fn set_cr3(pml4: PhysicalAddress){
    unsafe {
        asm!(
            "mov cr3, {0}",
            in(reg) pml4
        )
    }    
}

const TABLE_ENTRY_COUNT : usize = 512;

extern "C"{
    static MAX_PHY_BIT : u8;
    static MAX_VRT_BIT : u8;
    static __KERNEL_END__ : VirtualAddress;
    static STACK_TOP : VirtualAddress;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct TableEntry(u64);

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
    tmp_entry_address: VirtualAddress,
    ready : bool
}
pub type VirtualAddress = u64;


//Todo fix this cause each "thread" will most likely have their own PTM
static  PAGE_TABLE_MANAGER : Spinlock<PageTableManager> = Spinlock::new(PageTableManager::new());

#[derive(Debug)]
pub enum PtmError {
    NoMapping(VirtualAddress),
    InvalidRange,
    IncorrectPageSize,
    UnallignedPage,
    PmmError(PmmError)
}

fn split_virtual(virt : VirtualAddress) -> ([u16; 4], u16){
    ([
        (virt >> 39) as u16 & 0x1ff,
        (virt >> 30) as u16 & 0x1ff,
        (virt >> 21) as u16 & 0x1ff,
        (virt >> 12) as u16 & 0x1ff,

    ], virt as u16 & 0x7ff)
}

fn next_page(mut addr : u64) -> u64{
    if addr % PAGE_SIZE != 0{
        addr += PAGE_SIZE - (addr%PAGE_SIZE);
    }
    addr
}

fn sign_extend(i : VirtualAddress) -> VirtualAddress{
    let sign = i & unsafe{1<<MAX_PHY_BIT};
    if sign != 0 {
        let mask = ((0-1) as u64) ^ (unsafe{ (1<<MAX_PHY_BIT-1)-1 });
        i | mask
    }
    else{
        i
    }

    
}

fn from_indexes(indexes : ([u16; 4], u16)) -> VirtualAddress{
    /*
    ([
        (virt >> 39) as u16 & 0x1ff,
        (virt >> 30) as u16 & 0x1ff,
        (virt >> 21) as u16 & 0x1ff,
        (virt >> 12) as u16 & 0x1ff,

    ], virt as u16 & 0xeff)
     */
    (indexes.0[0] as u64).shl(39) | (indexes.0[1] as u64).shl(30) | (indexes.0[2] as u64).shl(21) | (indexes.0[3] as u64).shl(12) | (indexes.1 as u64)
}

impl PageTableManager{
    pub(self) const fn new() -> Self{
        PageTableManager {phy_addr : 0, tmp_address : 0, tmp_entry_address : 0,  ready : false}
    } 

    fn dump_to_serial(top : PhysicalAddress){
        let mut serial = Serial::new_uninit(0x3F8);
        
        fn ds_recurse(layer : VirtualAddress, serial : &mut Serial, depth : usize){
            if layer == 0{
                return;
            }
            let entries = layer as *mut TableEntry;
            serial.write(b'[');
            let mut previous = false;
            for i in 0..512{
                let entry = unsafe {
                    entries.add(i).read()
                };
                match entry.get_pointer(){
                    0 => continue,
                    x => {
                        if previous{
                            serial.write(b',');
                        }
                        serial.write(b'{');
                        let old_tmp = PageTableManager::get_temp();
                        let next = PageTableManager::map_temp(x).unwrap_or(0);
                        fmt::write(serial, format_args!("\"flags\":{},\"address\":\"{:x}\",\"depth\":{}", entry.get_flags(), entry.get_pointer(), depth)).unwrap();
                        if depth != TableLayer::PT.into(){
                            fmt::write(serial, format_args!(",\"layer\":")).unwrap();
                            ds_recurse(next, serial, depth+1);
                        }
                        PageTableManager::map_temp(old_tmp).unwrap();
                        serial.write(b'}');
                        previous = true;
                    }
                }
            }
            serial.write(b']');
        }
        let old_tmp = PageTableManager::get_temp();
        let next = PageTableManager::map_temp(top).unwrap_or(0);
        ds_recurse(next, &mut serial, 0);
        serial.write(b'\n');
        PageTableManager::map_temp(old_tmp).unwrap();
    }

    pub fn init() -> Result<(), PtmError>{
        
        let pg = PMM::pop_page();
        let mut guard = PAGE_TABLE_MANAGER.lock();
        let ptm = guard.get_mut();
        ptm.phy_addr = pg;
        drop(guard);
        
        let kaddr = KERNEL_REQ.get_response().unwrap();
        let kend = next_page(core::ptr::addr_of!(__KERNEL_END__) as VirtualAddress);
        let curr = kaddr.virtual_base();
        let curr_phy = kaddr.physical_base();
        
        let klen = kend-curr;
        Self::map_range(curr_phy..=curr_phy+klen, curr..=curr+klen, paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT)?;
        

        // this is the address that will be given to the user when they call map_temp
        let tmp_space = kend+PAGE_SIZE;
        let _test = Self::crawl_alloc(tmp_space, TableLayer::PT, paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
        unsafe {
            let mut t = _test.read();
            t.set_pointer(0xdeadbeef);
            t.set_flags(0);
            _test.write(t);
        }
        let tmp_space_pdt = Self::crawl(tmp_space, TableLayer::PDT)?;
        
        // which means we need to make it modifiable without re-mapping or else we get crazy infinite recurrsion 
        // we will use the next virtual page to map to the tmp space

        let _hhdm_offs = limine_req::HHDM_REQ.get_response().unwrap().offset();
        let tmp_space_idx = split_virtual(tmp_space).0[3];



        let tmp_page = tmp_space+PAGE_SIZE; 
        let ptmp_page_entry = Self::crawl_alloc(tmp_page, TableLayer::PT, paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
        unsafe {
            let mut tmp_page_entry = ptmp_page_entry.read();
            let ptr = tmp_space_pdt.read().get_pointer();
            tmp_page_entry.set_pointer(ptr);
            tmp_page_entry.set_flags(paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
            ptmp_page_entry.write(tmp_page_entry);
        }         

        Self::dump_to_serial(pg);

        let mut guard  =PAGE_TABLE_MANAGER.lock();
        let ptm = guard.get_mut();
        ptm.tmp_address = tmp_space;
        ptm.tmp_entry_address = tmp_page + (tmp_space_idx as usize * size_of::<TableEntry>()) as u64;
        ptm.ready = true;
            
        set_cr3(ptm.phy_addr);
        Ok(())
    }

    pub extern "sysv64" fn map(phy_addr : PhysicalAddress, virt_addr : VirtualAddress, flags : u64) -> i8{
        if phy_addr % PAGE_SIZE != 0{
            -1
        }
        else{
            // let mut guard = PAGE_TABLE_MANAGER.lock();
            // let ptm = guard.get_mut();
            let pentry = PageTableManager::crawl_alloc(virt_addr, TableLayer::PT, flags);
            let mut entry = unsafe{pentry.read()};
            entry.set_pointer(phy_addr);
            entry.set_flags(flags);
            
            
            unsafe {
                invalidate_page(virt_addr);
                pentry.write(entry);
            };
            0
        }
    }
    pub fn map_range(phy : RangeInclusive<PhysicalAddress>, virt : RangeInclusive<VirtualAddress>, flags : u64) -> Result<(), PtmError>{
        
        if phy.clone().count() != virt.clone().count(){
            Err(PtmError::InvalidRange)
        }
        else if (phy.end()-phy.start()) % PAGE_SIZE != 0{
            Err(PtmError::IncorrectPageSize)
        }
        else if phy.start() % PAGE_SIZE != 0{
            Err(PtmError::UnallignedPage)
        }
        else {
            let pi = phy.step_by(PAGE_SIZE as usize);
            let vi = virt.step_by(PAGE_SIZE as usize);
            for (p, v) in pi.zip(vi){
                
                if PageTableManager::map(p, v, flags) == -1{
                    return Err(PtmError::UnallignedPage);
                }

            }
            Ok(())
        }
    }

    pub fn map_temp(phy_addr : PhysicalAddress) -> Result<VirtualAddress, PtmError>{
        let guard = PAGE_TABLE_MANAGER.lock();
        if !guard.get().ready{
            let offset = HHDM_REQ.get_response().unwrap().offset();
            Ok(phy_addr + offset)
        }
        else {
            let ret = guard.get().tmp_address;
            let ppte = guard.get().tmp_entry_address as *mut TableEntry;
            drop(guard);

            unsafe {
                let mut pte = ppte.read();
                pte.set_pointer(phy_addr);
                pte.set_flags(paging_flags::PAGING_RW | paging_flags::PAGING_PRESENT);
                ppte.write(pte);
                invalidate_page(ret);
            }
            Ok(ret)    
            
        }
    }
    pub fn unmap_temp(){
        let mut guard = PAGE_TABLE_MANAGER.lock();
        let ptm = guard.get_mut();
        if ptm.ready{
            let ppte = ptm.tmp_entry_address as *mut TableEntry;
            unsafe {
                let mut pte = ppte.read();
                pte.set_pointer(0);
                pte.set_flags(0);
            }
        }

    }

    pub fn get_physical(virt_addr : VirtualAddress) -> Result<PhysicalAddress, PtmError>{
        if !PAGE_TABLE_MANAGER.lock().get().ready{
            
            let offset = HHDM_REQ.get_response().unwrap().offset();

            Ok(virt_addr-offset)
        }
        else{

            let res = Self::crawl(virt_addr, TableLayer::PT);
            match res{
                Ok(x) => {
                    let entry = unsafe{x.read()};
                    Ok(entry.get_pointer())
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
    }
    pub fn get_temp() -> PhysicalAddress{
        if !PAGE_TABLE_MANAGER.lock().get().ready{
            0
        }
        else {
            let tmp = PAGE_TABLE_MANAGER.lock().get().tmp_entry_address as *const TableEntry;
            unsafe {
                tmp.read().get_pointer()
            }
        }
    }

    pub(self) fn crawl(virt : VirtualAddress, layer : TableLayer) -> Result<*mut TableEntry, PtmError>{
        let (indexes, _) = split_virtual(virt);
        let phy_addr = PAGE_TABLE_MANAGER.lock().get().phy_addr;
        let mut ret = PageTableManager::map_temp(phy_addr)? as *mut TableEntry;
        let l : usize = layer.into(); 

        for i in 0..l{
            let current = unsafe {ret.add(indexes[i] as usize).read()};
            let phy = current.get_pointer();
            if phy == 0{
                return Err(PtmError::NoMapping(virt));
            }
            ret = PageTableManager::map_temp(phy)? as *mut TableEntry; 
        }
        Ok(unsafe{ret.add(indexes[l].into())})
    }

    pub(self) fn crawl_alloc(virt : VirtualAddress, table_layer : TableLayer, flags : u64) -> *mut TableEntry{
        let (indexes, _) = split_virtual(virt);
        let phy_addr = PAGE_TABLE_MANAGER.lock().get().phy_addr;

        let mut ret = PageTableManager::map_temp(phy_addr).unwrap() as *mut TableEntry;
        let l : usize = table_layer.into(); 

        for i in 0..l{
            
            let mut entry = unsafe{ret.add(indexes[i] as usize).read()};
            entry.check_and_alloc(flags);
            unsafe {ret.add(indexes[i] as usize).write(entry)}
            ret = PageTableManager::map_temp(entry.get_pointer()).unwrap() as *mut TableEntry; 
        }
        unsafe{ret.add(indexes[l].into())}
    }
}
impl TableEntry {

    pub fn generate_phy_mask() -> u64{
        unsafe {
            let upper = (1<<(MAX_PHY_BIT-1))-1;
            let lower = (1<<11)-1;
            upper ^ lower
        }
    }

    pub fn new(phy : PhysicalAddress, rw : bool) -> TableEntry{
        let phy_mask = Self::generate_phy_mask();
        let mut val = 0;
        val |= (rw as u64) << 1;
        val |= (phy << 12) & phy_mask;
        TableEntry(val)
    }
    pub fn get_pointer(&self) -> PhysicalAddress{
        let phy_mask = Self::generate_phy_mask();
        self.0 & phy_mask
    }
    
    pub fn set_pointer(&mut self, ptr : PhysicalAddress){
        let phy_mask = Self::generate_phy_mask();
        self.0 &= !phy_mask;
        self.0 |= ptr & phy_mask;
    }

    pub fn get_flags(&self) -> u64{
        let phy_mask = Self::generate_phy_mask();
        self.0 & !phy_mask
    }


    pub fn set_flags(&mut self, flags : u64){
        let phy_mask = Self::generate_phy_mask();
        self.0 &= phy_mask;
        self.0 |= flags;
    }
}


impl TableEntry{
    pub fn check_and_alloc(&mut self, flags : u64){
        let present = (self.0 & paging_flags::PAGING_PRESENT) != 0;
        if present{
            self.set_flags(flags);
        }
        else{
            let page = PMM::pop_page();
            self.set_pointer(page);
            self.set_flags(flags);
        }
    }
}

impl From<TableEntry> for u64{
    fn from(value: TableEntry) -> Self {
        value.0
    }
}

impl From<u64> for TableEntry{
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<TableLayer> for usize{
    fn from(value: TableLayer) -> Self {
        match value {
            TableLayer::PML4 => 0,
            TableLayer::PDPT => 1,
            TableLayer::PDT => 2,
            TableLayer::PT => 3,
        }
    }
}


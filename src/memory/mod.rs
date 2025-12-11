pub mod alloc;
pub mod paging;
pub mod pmm;
//chmod allocator;
pub const PAGE_SIZE: u64 = 4096;
pub const PS_USIZE: usize = PAGE_SIZE as usize;

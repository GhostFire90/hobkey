use core::ffi::CStr;

use limine::cstr;


#[derive(Clone, Copy)]
#[repr(packed)]
#[allow(dead_code)]
pub struct UstarHeader{
    pub filename : [u8; 100],
    pub mode : u64,
    pub owner_id : u64,
    pub group_id : u64,
    pub size_ascii_octal : [u8;12],
    pub last_modified : [u8;12],
    pub checksum : u64,
    pub file_type : u8,
    pub linked_name : [u8; 100],
    pub mag : [u8;6],
    pub version : [u8;2],
    pub owner_name : [u8;32],
    pub owner_group_name : [u8;32],
    pub device_major : u64,
    pub device_minor : u64,
    pub filename_prefix : [u8;155],
    pub _padding : [u8;12]
}
pub enum UstarFileType{
    Normal,
    SymLink,
    CharDev,
    BlockDev,
    Dir,
    FifoPipe,
    Unknown
}
impl Into<u8> for UstarFileType{
    fn into(self) -> u8 {
        match self  {
            UstarFileType::Normal   => 0,
            UstarFileType::SymLink  => 1,
            UstarFileType::CharDev  => 2,
            UstarFileType::BlockDev => 3,
            UstarFileType::Dir      => 4,
            UstarFileType::FifoPipe => 5,
            UstarFileType::Unknown  => 6,
        }
    }
}
impl From<u8> for UstarFileType{
    fn from(value: u8) -> Self {
        match value {
            0 => UstarFileType::Normal,
            1 => UstarFileType::SymLink,
            2 => UstarFileType::CharDev,
            3 => UstarFileType::BlockDev,
            4 => UstarFileType::Dir,
            5 => UstarFileType::FifoPipe,
            _ => UstarFileType::Unknown
        }
    }
}
#[derive(Debug)]
pub enum UstarError{
    FileNotFound,
    IncorrectFormat
}




pub fn find_file(filename : &str, arc : *const u8, len : usize) -> Result<(UstarHeader, usize), UstarError>{

    let mut current_idx = 0;
    const CORRECT_MAG : &core::ffi::CStr= cstr!("ustar");


    while current_idx < len{
        let header = unsafe {arc.add(current_idx).cast::<UstarHeader>().read()};
        if header.mag == CORRECT_MAG.to_bytes_with_nul(){
            let mut full_name : [u8; 255] = [0;255];
            let pre = header.filename_prefix.iter().take_while(|x| **x != 0);
            let main = header.filename.iter().take_while(|x| **x != 0);
            let mut full = pre.chain(main);
            
            for i in 0..255{
                let onext = full.next();
                if let Some(next) = onext{
                    full_name[i] = *next;
                }
                else{
                    break;
                }
            };
            
            if CStr::from_bytes_until_nul(&full_name).unwrap().to_str().unwrap() == filename{
                return Ok((header, current_idx+512));
            }

            current_idx += 512 + crate::helpers::atoi(&header.size_ascii_octal, 8).ok_or(UstarError::IncorrectFormat)?;
        }
        else {
            return Err(UstarError::IncorrectFormat)
        }
    }    
    Err(UstarError::FileNotFound)
}
use core::{ffi::CStr, marker::PhantomData, ptr::NonNull};

use alloc::slice;

const USTAR_MAG: &'static [u8] = "ustar\0".as_bytes();

#[derive(Clone, Copy)]
#[repr(packed)]
#[allow(dead_code)]
pub struct UstarHeader
{
  pub filename: [u8; 100],
  pub mode: u64,
  pub owner_id: u64,
  pub group_id: u64,
  pub size_ascii_octal: [u8; 12],
  pub last_modified: [u8; 12],
  pub checksum: u64,
  pub file_type: u8,
  pub linked_name: [u8; 100],
  pub mag: [u8; 6],
  pub version: [u8; 2],
  pub owner_name: [u8; 32],
  pub owner_group_name: [u8; 32],
  pub device_major: u64,
  pub device_minor: u64,
  pub filename_prefix: [u8; 155],
  pub _padding: [u8; 12],
}
impl UstarHeader
{
  pub fn validate(&self) -> bool
  {
    self.mag == USTAR_MAG
  }
  pub fn file_length(&self) -> usize
  {
    self
      .size_ascii_octal
      .iter()
      .cloned()
      .take(11)
      .map(|x| x - b'0')
      .rev()
      .enumerate()
      .map(|(index, digit)| 8_usize.pow(index as u32) * digit as usize)
      .sum()
  }
  pub fn file_name(&self) -> ([u8; 255], usize)
  {
    let mut full: [u8; 255] = [0; 255];
    let mut size = 0;
    self
      .filename_prefix
      .iter()
      .take_while(|x| **x != 0)
      .chain(self.filename.iter().take_while(|x| **x != 0))
      .for_each(|x| {
        full[size] = *x;
        size += 1;
      });

    (full, size)
  }
  pub fn compare_names(&self, rhs: &str) -> bool
  {
    let my_iter = self
      .filename_prefix
      .iter()
      .take_while(|x| **x != 0)
      .chain(self.filename.iter().take_while(|x| **x != 0));
    let count = my_iter.clone().count();
    if count != rhs.len()
    {
      return false;
    }
    my_iter.zip(rhs.as_bytes().iter()).all(|(a, b)| *a == *b)
  }
}

#[repr(u8)]
pub enum UstarFileType
{
  Normal,
  SymLink,
  CharDev,
  BlockDev,
  Dir,
  FifoPipe,
  Unknown,
}

#[derive(Debug)]
pub enum UstarError
{
  FileNotFound,
  IncorrectFormat,
}

pub struct UstarArchive<'a>
{
  data: NonNull<u8>,

  length: usize,

  _boo: PhantomData<&'a [u8]>,
}

pub struct UstarIter<'a>
{
  next: Option<NonNull<u8>>,
  _boo: PhantomData<&'a [u8]>,
}
impl<'a> UstarArchive<'a>
{
  pub fn new(data: NonNull<u8>, length: usize) -> Self
  {
    Self {
      data,
      length,
      _boo: PhantomData,
    }
  }
  pub fn iter(&self) -> UstarIter<'a>
  {
    UstarIter {
      next: Some(self.data),
      _boo: PhantomData,
    }
  }
}

impl<'a> Iterator for UstarIter<'a>
{
  type Item = (UstarHeader, &'a [u8]);

  fn next(&mut self) -> Option<Self::Item>
  {
    if let Some(p_data) = self.next
    {
      let header = unsafe { *(p_data.as_ptr() as *const UstarHeader) };
      if !header.validate()
      {
        return None;
      }
      self.next = Some(unsafe {
        p_data.byte_add(size_of::<UstarHeader>() + header.file_length().next_multiple_of(512))
      });
      Some((header, unsafe {
        slice::from_raw_parts(
          p_data.as_ptr().add(size_of::<UstarHeader>()),
          header.file_length(),
        )
      }))
    }
    else
    {
      None
    }
  }
}

impl<'a> UstarIter<'a>
{
  pub fn find_file(&mut self, name: &str) -> Option<(UstarHeader, &'a [u8])>
  {
    self.find(|(header, _)| header.compare_names(name))
  }
}

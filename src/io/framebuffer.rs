use core::{
  fmt::{self, Write},
  marker::PhantomData,
  ptr::NonNull,
};

use alloc::slice;

use crate::io::psf::{Glyph, Psf};

pub struct Framebuffer<'a>
{
  address: NonNull<u8>,
  pitch: usize,
  width: usize,
  height: usize,

  size: usize,

  fg_color: u32,
  bg_color: u32,

  font: Option<Psf>,
  /// not pixel positions, its character positions
  cursor_pos: (usize, usize),

  _boo: PhantomData<&'a [u8]>,
}

impl<'a> Framebuffer<'a>
{
  #[inline]
  pub const fn new(address: NonNull<u8>, width: usize, height: usize, pitch: usize) -> Self
  {
    Self {
      address,
      pitch,
      width,
      height,
      size: pitch * height,
      font: None,
      fg_color: u32::MAX,
      bg_color: 0,
      cursor_pos: (0, 0),
      _boo: PhantomData,
    }
  }
  #[inline]
  pub fn set_font(&mut self, font: Psf) -> Option<Psf>
  {
    self.font.replace(font)
  }

  #[inline]
  pub fn set_cursor_x(&mut self, x: usize)
  {
    self.cursor_pos.0 = x
  }
  #[inline]
  pub fn set_cursor_y(&mut self, y: usize)
  {
    self.cursor_pos.1 = y
  }
  #[inline]
  pub fn set_cursor_pos(&mut self, pos: (usize, usize))
  {
    self.cursor_pos = pos
  }

  #[inline]
  pub fn get_bytes(&self) -> &'a [u8]
  {
    unsafe { slice::from_raw_parts(self.address.as_ptr(), self.size) }
  }
  #[inline]
  pub fn get_bytes_mut(&mut self) -> &'a mut [u8]
  {
    unsafe { slice::from_raw_parts_mut(self.address.as_ptr(), self.size) }
  }
}

impl<'a> Write for Framebuffer<'a>
{
  fn write_str(&mut self, s: &str) -> core::fmt::Result
  {
    if self.font.is_none()
    {
      return Err(fmt::Error);
    }

    let font = self.font.as_ref().unwrap();
    let bytes = Glyph::expand_str(
      &font.glyphs(s).map_err(|_| fmt::Error)?,
      self.fg_color,
      self.bg_color,
    );
    let glyph_count = s.len();
    let glyph_width = font.width();
    let glyph_height = font.height();
    let start_pos = self.cursor_pos.0 * glyph_width + glyph_height * self.pitch * self.cursor_pos.1;
    let pixels = unsafe {
      slice::from_raw_parts_mut(
        self.address.as_ptr() as *mut u32,
        self.size / size_of::<u32>(),
      )
    };

    for i in 0..bytes.len()
    {
      let row = i / (glyph_width * glyph_count);
      let col = i % (glyph_width * glyph_count);
      let pos = row * (self.pitch / size_of::<u32>()) + col;
      pixels[pos + start_pos] = bytes[i];
    }
    self.cursor_pos.0 += glyph_count;
    Ok(())
  }
}

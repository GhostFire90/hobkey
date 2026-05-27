use core::marker::PhantomData;
use core::ptr::NonNull;

use alloc::vec;
use alloc::vec::Vec;

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct Psf1
{
  /// Hex: `36 04`
  _magic: [u8; 2],

  /// # [See font modes](https://en.wikipedia.org/wiki/PC_Screen_Font#Font_modes)
  modes: u8,

  /// glyph height
  glyph_size: u8,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
struct Psf2
{
  /// Hex: `72 b5 4a 86`
  _magic: [u8; 4],

  /// Always 0 5/23/26
  _version: u32,

  header_size: u32,

  /// # [See flags](https://en.wikipedia.org/wiki/PC_Screen_Font#Font_flags)
  flags: u32,

  /// Number of glyphs
  length: u32,

  /// height of glyphs
  height: u32,

  /// size of glyph in bytes
  glyph_size: u32,

  /// width of glyphs,
  width: u32,
}
#[derive(Debug, Clone, Copy)]
pub enum PsfError
{
  InvalidHeader,
  InvalidGlyph,
}

enum PsfHeader
{
  Psf1(Psf1),
  Psf2(Psf2),
}

impl Psf1
{
  const MAG: [u8; 2] = [0x36, 0x04];
}
impl Psf2
{
  const MAG: [u8; 4] = [0x72, 0xb5, 0x4a, 86];
}

impl PsfHeader
{
  fn is_psf1(data: NonNull<u8>) -> bool
  {
    let mag: [u8; 2] = unsafe { (data.as_ptr() as *const [u8; 2]).read() };
    mag.eq(&Psf1::MAG)
  }
  fn is_psf2(data: NonNull<u8>) -> bool
  {
    let mag: [u8; 4] = unsafe { (data.as_ptr() as *const [u8; 4]).read() };
    mag.eq(&Psf2::MAG)
  }
  pub fn parse(data: NonNull<u8>) -> Result<Self, PsfError>
  {
    if Self::is_psf1(data)
    {
      Ok(Self::Psf1(unsafe { *(data.as_ptr() as *const Psf1) }))
    }
    else if Self::is_psf2(data)
    {
      Ok(Self::Psf2(unsafe { *(data.as_ptr() as *const Psf2) }))
    }
    else
    {
      Err(PsfError::InvalidHeader)
    }
  }

  #[inline]
  pub fn glyph_width(&self) -> usize
  {
    match self
    {
      PsfHeader::Psf1(_) => 8,
      PsfHeader::Psf2(psf2) => psf2.width as usize,
    }
  }
  #[inline]
  pub fn glyph_height(&self) -> usize
  {
    match self
    {
      PsfHeader::Psf1(psf1) => psf1.glyph_size as usize,
      PsfHeader::Psf2(psf2) => psf2.height as usize,
    }
  }
  #[inline]
  pub fn glyph_dimensions(&self) -> (usize, usize)
  {
    (self.glyph_width(), self.glyph_height())
  }
}

/// Glyph type, just bitmasks
pub struct Glyph<'a>
{
  pub bitmask: &'a [u8],
  pub width: usize,
  pub height: usize,
}

pub struct Psf<'a>
{
  pub data: NonNull<u8>,

  header: PsfHeader,
  _boo: PhantomData<Glyph<'a>>,
}

impl<'a> Psf<'a>
{
  /// # SAFETEY!!!!
  /// `data` must have a lifetime >= the resultant Self. </br>
  /// Treat the bytes stored in `data` as if they are owned by the result
  pub fn new(data: NonNull<u8>) -> Result<Self, PsfError>
  {
    let header = PsfHeader::parse(data)?;
    let header_size = match &header
    {
      PsfHeader::Psf1(_) => size_of::<Psf1>(),
      PsfHeader::Psf2(_) => size_of::<Psf2>(),
    };

    Ok(Self {
      data: unsafe { data.byte_add(header_size) },
      header,
      _boo: PhantomData,
    })
  }
  pub fn is_unicode(&self) -> bool
  {
    // TODO! FIXME PLEASE I BEG YOU
    false
  }

  pub fn get_glyph_count(&self) -> usize
  {
    match &self.header
    {
      PsfHeader::Psf1(psf1) =>
      {
        if psf1.modes & 0x1 == 0
        {
          256
        }
        else
        {
          512
        }
      }
      PsfHeader::Psf2(psf2) => psf2.length as usize,
    }
  }

  pub fn glyph_stride(&self) -> usize
  {
    match &self.header
    {
      PsfHeader::Psf1(psf1) => psf1.glyph_size as usize,
      PsfHeader::Psf2(psf2) => psf2.glyph_size as usize,
    }
  }

  /// array of bitmasks, row length = `self.width().next_multiple_of(8)`
  pub fn get_glyph(&self, character: char) -> Result<Glyph, PsfError>
  {
    if !character.is_ascii()
    {
      return Err(PsfError::InvalidGlyph);
    }
    if character.is_control()
    {
      return self.get_glyph(' ');
    }

    let stride = self.glyph_stride();
    let offset = character as usize * stride;
    Ok(Glyph {
      bitmask: unsafe {
        core::slice::from_raw_parts(self.data.as_ptr().byte_add(offset), stride).into()
      },
      width: self.width(),
      height: self.height(),
    })
  }

  pub fn glyphs(&self, s: &str) -> Result<Vec<Glyph>, PsfError>
  {
    s.chars().into_iter().map(|x| self.get_glyph(x)).collect()
  }

  /// Width of each glyph in pixels
  #[inline]
  pub fn width(&self) -> usize
  {
    self.header.glyph_width()
  }
  /// Height of each glyph in pixels
  #[inline]
  pub fn height(&self) -> usize
  {
    self.header.glyph_height()
  }

  #[inline]
  pub fn dimensions(&self) -> (usize, usize)
  {
    self.header.glyph_dimensions()
  }
}

impl<'a> Glyph<'a>
{
  /// Expands self into a RGB color array,
  #[inline]
  pub fn expand(&self, fg_color: u32, bg_color: u32) -> Vec<u32>
  {
    let row_stride = self.width.next_multiple_of(8) / 8;
    let rows = self.bitmask.chunks(row_stride);
    rows
      .into_iter()
      .map(|row| {
        let mut expanded = vec![];
        for x in 0..self.width
        {
          let actual = self.width - x - 1;
          let byte = row[actual / 8];
          let on = (byte >> (actual % 8)) & 0x1 == 1;
          if on
          {
            expanded.push(fg_color);
          }
          else
          {
            expanded.push(bg_color);
          }
        }
        expanded
      })
      .flatten()
      .collect()
  }

  #[inline]
  fn expand_bitmask(bitmask: u8, fg_color: u32, bg_color: u32) -> Vec<u32>
  {
    let mut res = vec![bg_color; 8];
    for i in 0..8
    {
      let on = (bitmask >> (8 - i - 1)) & 0x1 == 1;
      if on
      {
        res[i] = fg_color;
      }
    }
    res
  }

  #[inline]
  pub fn expand_str(glyphs: &[Self], fg_color: u32, bg_color: u32) -> Vec<u32>
  {
    if glyphs.is_empty()
    {
      return vec![];
    }

    // how many characters
    let glyph_count = glyphs.len();

    // height of each glyph
    let glyph_height = glyphs[0].height;

    // the stride between rows in an individual glyph
    let glyph_row_stride = glyphs[0].width.next_multiple_of(8) / 8;

    // Length of row in bytes
    let row_length = glyphs[0].width * glyph_count;

    // length of final buffer
    let total_bytes = row_length * glyph_height;

    // return pixels
    let mut ret = Vec::with_capacity(total_bytes);

    for row in 0..glyph_height
    {
      let start = row * glyph_row_stride;
      for col in 0..glyph_count
      {
        let glyph = &glyphs[col];
        let mut bytes = glyph.bitmask[start..start + glyph_row_stride]
          .iter()
          .flat_map(|x| Self::expand_bitmask(*x, fg_color, bg_color))
          .collect();
        ret.append(&mut bytes);
      }
    }

    ret
  }
}

use alloc::collections::BTreeSet;

use crate::memory::{paging::VirtualAddress, MemoryRange, PAGE_SIZE};

pub struct MmapTracker
{
  ranges: BTreeSet<MemoryRange>,
}

impl MmapTracker
{
  pub fn new(base: VirtualAddress, max: VirtualAddress) -> Self
  {
    let mut ranges = BTreeSet::new();
    ranges.insert((base..max).into());
    Self { ranges }
  }

  pub fn get_mapping(&mut self, length: usize) -> Option<MemoryRange>
  {
    let page_length = length.next_multiple_of(PAGE_SIZE as usize);
    let entry = self.ranges.iter().find(|x| x.len() >= page_length)?.clone();
    self.ranges.remove(&entry);
    let (ret, new) = entry.shrink(page_length)?;

    if new.len() > 0
    {
      self.ranges.insert(new);
    }

    Some(ret)
  }

  pub fn release_mapping(&mut self, mapping: MemoryRange)
  {
    let pos = self.ranges.iter().position(|x| *x < mapping).unwrap_or(0);

    let (left, right) = (
      self.ranges.iter().nth(pos.wrapping_sub(1)).cloned(),
      self.ranges.iter().nth(pos.wrapping_add(1)).cloned(),
    );

    /*
     * Monads go brrt, this is a memory range merge
     * 1. if left exists try and merge it with the found entry
     *  a. if merge succeeds, remove left from the map
     *  b. else, just return the entry
     * 2. try to merge the result of last step with right entry if it exists
     *  - follow same steps as previous step with rhs
     */
    let new_left = left
      .and_then(|lhs| {
        lhs.append(&mapping).and_then(|x| {
          self.ranges.remove(&lhs);
          Some(x)
        })
      })
      .unwrap_or(mapping);
    let entry = right
      .and_then(|rhs| {
        new_left.append(&rhs).and_then(|x| {
          self.ranges.remove(&rhs);
          Some(x)
        })
      })
      .unwrap_or(new_left);
    self.ranges.insert(entry);
  }
}

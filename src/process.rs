use crate::{
  memory::paging::{PageTableManager, PtmError},
  spinlock::Spinlock,
};

pub struct Process
{
  id: usize,
  pub ptm: Spinlock<PageTableManager>,
}

pub static CURRENT_PROC: Spinlock<Option<Process>> = Spinlock::new(None);

impl Process
{
  pub fn new(ptm: PageTableManager) -> Self
  {
    // TODO, id increments
    Self {
      id: 0,
      ptm: Spinlock::new(ptm),
    }
  }

  pub fn id(&self) -> usize
  {
    self.id
  }
  pub(crate) fn ptm_operation<T>(
    &self,
    function: impl FnOnce(&mut PageTableManager) -> Result<T, PtmError>,
  ) -> Result<T, PtmError>
  {
    let mut guard = self.ptm.lock();
    function(&mut guard)
  }
}

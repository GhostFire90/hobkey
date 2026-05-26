use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{self, AtomicBool};

pub struct Spinlock<T>
{
  stored: UnsafeCell<T>,
  lock: AtomicBool,
}
pub struct SpinlockGuard<'a, T>
{
  lock: &'a Spinlock<T>,
}

pub struct SpinlockOnce<T>
{
  init: AtomicBool,
  locked: AtomicBool,
  stored: UnsafeCell<Option<T>>,
}

unsafe impl<T: Send> Send for Spinlock<T> {}
unsafe impl<T: Send> Sync for Spinlock<T> {}

unsafe impl<T: Send> Send for SpinlockOnce<T> {}
unsafe impl<T: Send> Sync for SpinlockOnce<T> {}

impl<T> Spinlock<T>
{
  pub const fn new(value: T) -> Self
  {
    Spinlock {
      stored: UnsafeCell::new(value),
      lock: AtomicBool::new(false),
    }
  }

  pub fn lock(&self) -> SpinlockGuard<'_, T>
  {
    while self
      .lock
      .compare_exchange_weak(
        false,
        true,
        atomic::Ordering::Acquire,
        atomic::Ordering::Relaxed,
      )
      .is_err()
    {
      core::hint::spin_loop();
    }
    SpinlockGuard { lock: self }
  }
}

impl<T> Drop for SpinlockGuard<'_, T>
{
  fn drop(&mut self)
  {
    self.lock.lock.store(false, atomic::Ordering::Release);
  }
}

impl<T> SpinlockGuard<'_, T>
{
  pub fn get(&self) -> &T
  {
    unsafe { &*self.lock.stored.get() }
  }
  pub fn get_mut(&mut self) -> &mut T
  {
    unsafe { &mut *self.lock.stored.get() }
  }
}

impl<T> Deref for SpinlockGuard<'_, T>
{
  type Target = T;

  fn deref(&self) -> &Self::Target
  {
    self.get()
  }
}
impl<T> DerefMut for SpinlockGuard<'_, T>
{
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    self.get_mut()
  }
}

impl<T> SpinlockOnce<T>
{
  pub const fn new() -> Self
  {
    Self {
      init: AtomicBool::new(false),
      locked: AtomicBool::new(false),
      stored: UnsafeCell::new(None),
    }
  }

  pub fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T
  {
    if self.init.load(atomic::Ordering::Acquire)
    {
      &unsafe { &*self.stored.get() }.as_ref().unwrap()
    }
    else
    {
      while self
        .locked
        .compare_exchange_weak(
          false,
          true,
          atomic::Ordering::Acquire,
          atomic::Ordering::Relaxed,
        )
        .is_err()
      {
        core::hint::spin_loop();
      }
      if !self.init.load(atomic::Ordering::Acquire)
      {
        unsafe { (*self.stored.get()).replace(f()) };
        self.init.store(true, atomic::Ordering::Release);
      }
      self.locked.store(false, atomic::Ordering::Release);
      &unsafe { &*self.stored.get() }.as_ref().unwrap()
    }
  }
  pub fn get(&self) -> Option<&T>
  {
    if self.init.load(atomic::Ordering::Acquire)
    {
      unsafe { &*self.stored.get() }.as_ref()
    }
    else
    {
      None
    }
  }
}

pub struct LazySpinlock<T, F: Fn() -> T = fn() -> T>
{
  once: SpinlockOnce<T>,
  func: F,
}

impl<T, F: Fn() -> T + Clone> LazySpinlock<T, F>
{
  pub const fn new(f: F) -> Self
  {
    LazySpinlock {
      once: SpinlockOnce::new(),
      func: f,
    }
  }
  pub fn get(&self) -> &T
  {
    self.once.get_or_init(self.func.clone())
  }
}

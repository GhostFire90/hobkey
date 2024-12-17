use core::sync::atomic::{self, AtomicBool};
use core::cell::UnsafeCell;

pub struct Spinlock<T>{
    stored : UnsafeCell<T>,
    lock : atomic::AtomicBool
}
pub struct SpinlockGuard<'a, T>{
    lock: &'a Spinlock<T>
}

unsafe impl<T: Send> Send for Spinlock<T> {}
unsafe impl<T: Send> Sync for Spinlock<T> {}


impl<T> Spinlock<T>{
    pub const fn new(value : T) -> Self{
        Spinlock { stored: UnsafeCell::new(value), lock: AtomicBool::new(false) }
    }

    pub fn lock(&self) -> SpinlockGuard<T>{
        while self.lock.compare_exchange_weak(false, true, atomic::Ordering::Acquire, atomic::Ordering::Relaxed).is_err(){}
        SpinlockGuard { lock: self }
    }
}

impl<T> Drop for SpinlockGuard<'_, T>{
    fn drop(&mut self) {
        self.lock.lock.store(false, atomic::Ordering::Release);
    }
}

impl<T> SpinlockGuard<'_, T>{
    pub fn get(&self) -> &T{
        unsafe { &*self.lock.stored.get()}
    }
    pub fn get_mut(&mut self) -> &mut T{
        unsafe {&mut *self.lock.stored.get()}
    }
}
use crate::std::{
    cell::UnsafeCell,
    fmt,
    ops::{Deref, DerefMut},
};
use crate::sys::{coreinit, ffi};
use crate::{Error, Result};

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    mutex: UnsafeCell<coreinit::mutex::Mutex>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        let mut mutex = coreinit::mutex::Mutex::new();

        unsafe {
            coreinit::mutex::init(&mut mutex);
        }

        Self {
            data: UnsafeCell::new(value),
            mutex: UnsafeCell::new(mutex),
        }
    }

    /// Acquires a mutex, blocking the current thread until it is able to do so.
    pub fn lock(&self) -> Result<MutexGuard<'_, T>> {
        unsafe {
            coreinit::mutex::lock(self.mutex.get());
        }
        Ok(MutexGuard::new(self))
    }

    /// Attempts to acquire this lock.
    pub fn try_lock(&self) -> Result<MutexGuard<'_, T>> {
        let locked = unsafe { coreinit::mutex::try_lock(self.mutex.get()) };
        match locked {
            ffi::TRUE => Ok(MutexGuard::new(self)),
            ffi::FALSE => Err(Error::Any("already locked")),
            _ => unreachable!(),
        }
    }

    fn unlock(&self) {
        unsafe {
            coreinit::mutex::unlock(self.mutex.get());
        }
    }
}

pub struct MutexGuard<'a, T: 'a> {
    lock: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> Self {
        Self { lock: mutex }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

impl<T: fmt::Debug> fmt::Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: fmt::Display> fmt::Display for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

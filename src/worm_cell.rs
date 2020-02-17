
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::rc::Rc;
use std::mem::MaybeUninit;
use std::ops::Deref;

use crate::error::*;

pub struct AtomicWormCell<T: Sized> {
    set_started: AtomicBool,
    set_completed: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>
}

unsafe impl<T> Sync for AtomicWormCell<T> {}

pub struct WormCell<T: Sized> {
    value: UnsafeCell<(bool, MaybeUninit<T>)>
}

pub struct AtomicWormCellReader<T: Sized> {
    wc: Arc<AtomicWormCell<T>>
}

pub struct WormCellReader<T: Sized> {
    wc: Rc<WormCell<T>>
}

impl<T: Sized> AtomicWormCell<T> {
    pub fn new() -> Self {
        AtomicWormCell {
            set_started: AtomicBool::new(false),
            set_completed: AtomicBool::new(false),
            value: UnsafeCell::<MaybeUninit<T>>::new(MaybeUninit::<T>::uninit())
        }
    }

    pub fn set(&self, val: T) {
        if self.set_started.compare_and_swap(false, true, Ordering::Relaxed) {
            panic!(WormCellError::ReadNotSet);
        }

        unsafe {
            let maybe = &mut *self.value.get();
            let dest = &mut *maybe.as_mut_ptr();
            *dest = val;
        }

        self.set_completed.store(true, Ordering::Relaxed);
    }

    pub fn try_set(&self, val: T) -> WormCellResult<()> {
        if self.set_started.compare_and_swap(false, true, Ordering::Relaxed) {
            return Err(WormCellError::DoubleSet)
        }

        unsafe {
            let maybe = &mut *self.value.get();
            let dest = &mut *maybe.as_mut_ptr();
            *dest = val;
        }

        self.set_completed.store(true, Ordering::Relaxed);

        Ok(())
    }

    pub fn get(&self) -> &T {
        if self.set_completed.load(Ordering::Relaxed) {
            unsafe {
                let maybe = &*self.value.get();
                &*maybe.as_ptr()
            }
        } else {
            panic!(WormCellError::ReadNotSet);
        }
    }

    pub fn try_get(&self) -> WormCellResult<&T> {
        if self.set_completed.load(Ordering::Relaxed) {
            unsafe {
                let maybe = &*self.value.get();
                Ok(&*maybe.as_ptr())
            }
        } else {
            Err(WormCellError::ReadNotSet)
        }
    }

    pub fn is_set(&self) -> bool {
        self.set_completed.load(Ordering::Relaxed)
    }
}


impl<T: Sized> WormCell<T> {
    pub fn new() -> Self {
        WormCell {
            value: UnsafeCell::<(bool, MaybeUninit<T>)>::new((false, MaybeUninit::<T>::uninit()))
        }
    }

    pub fn set(&self, val: T) {
        unsafe {
            let maybe = &mut *self.value.get();
            if maybe.0 {
                panic!(WormCellError::ReadNotSet);
            }
            
            let dest = &mut *maybe.1.as_mut_ptr();
                
            *dest = val;

            maybe.0 = true;
        }
    }

    pub fn try_set(&self, val: T) -> WormCellResult<()> {
        unsafe {
            let maybe = &mut *self.value.get();
            if maybe.0 {
                return Err(WormCellError::ReadNotSet);
            }
            
            let dest = &mut *maybe.1.as_mut_ptr();
                
            *dest = val;

            maybe.0 = true;
        }

        Ok(())
    }

    pub fn get(&self) -> &T {
        unsafe {
            let maybe = &*self.value.get();
            if maybe.0 {
                &*maybe.1.as_ptr()
            } else {
                panic!(WormCellError::ReadNotSet);
            }
        }
    }

    pub fn try_get(&self) -> WormCellResult<&T> {
        unsafe {
            let maybe = &*self.value.get();
            if maybe.0 {
                Ok(&*maybe.1.as_ptr())
            } else {
                Err(WormCellError::ReadNotSet)
            }
        }
    }

    pub fn is_set(&self) -> bool {
        unsafe {
            let maybe = &*self.value.get();
            maybe.0
        }
    }
}

impl<T: Sized> AtomicWormCellReader<T> {
    pub fn new(wc: Arc<AtomicWormCell<T>>) -> AtomicWormCellReader<T> {
        AtomicWormCellReader{ wc: wc }
    }
    
    pub fn get(&self) -> &T {
        
        (*self.wc).get()
    }

    pub fn try_get(&self) -> WormCellResult<&T> {
        self.wc.try_get()
    }
}

unsafe impl<T> Sync for AtomicWormCellReader<T> {}

impl<T: Sized> WormCellReader<T> {
    pub fn new<'a>(wc: Rc<WormCell<T>>) -> WormCellReader<T> {
        WormCellReader { wc: wc }
    }
    
    pub fn get(&self) -> &T {
        self.wc.get()
    }

    pub fn try_get(&self) -> WormCellResult<&T> {
        self.wc.try_get()
    }
}

impl<T: Sized> Deref for AtomicWormCellReader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> Deref for WormCellReader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

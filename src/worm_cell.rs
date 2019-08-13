use std::cell::UnsafeCell;

use crate::error::*;

pub struct WormCell<T: Sized> {
    value: Box<UnsafeCell<Option<T>>>
}

#[derive(Debug, Copy, Clone)]
pub struct WormCellReader<T: Sized> {
    value: *const Option<T>
}

impl<T> WormCell<T> {
    pub fn new() -> Self {
        WormCell { value: Box::new(UnsafeCell::<Option<T>>::new(None)) }
    }

    pub fn set(&self, val: T) -> WormCellResult<()> {
        let safer: &mut Option<T> = unsafe {
            &mut *self.value.get()
        };

        match safer {
            None => {
                *safer = Some(val);
                Ok(())
            }
            Some(_) => Err(WormCellError::DoubleSet)
        }
    }

    pub fn reader(&self) -> WormCellReader<T> {
        WormCellReader::<T> { value: self.value.get() }
    }

    pub fn is_set(&self) -> bool {
        let safer: &Option<T> = unsafe {
            &mut *self.value.get()
        };

        match safer {
            Some(_) => true,
            None => false
        }
    }
}

impl<T> WormCellReader<T> {
    pub fn get(&self) -> WormCellResult<&T> {
        match unsafe { &*self.value } {
            Some(ref val) => Ok(val),
            None => Err(WormCellError::ReadNotSet)
        }
    }
}

#[macro_use]
extern crate failure;

use std::cell::UnsafeCell;

#[derive(Fail, Debug)]
pub enum WormCellError {
    #[fail(display = "Tried to read a WormReader that wasn't set")]
    ReadNotSet,
    #[fail(display = "Tried to set a WormCell twice")]
    DoubleSet
}

pub type WormCellResult<T> = Result<T, WormCellError>;

pub struct WormCell<T: Sized> {
    value: UnsafeCell<Option<T>>
}

pub struct WormCellReader<'a, T: Sized> {
    value: &'a Option<T>
}

impl<'a, T> WormCell<T> {
    pub fn new() -> Self {
        WormCell { value: UnsafeCell::<Option<T>>::new(None) }
    }

    pub fn set(&self, val: T) {
        let safer: &mut Option<T> = unsafe {
            &mut *self.value.get()
        };

        match safer {
            Some(_) => panic!("Setting already set WormCell!"),
            None => *safer = Some(val)
        }
    }

    pub fn reader(&self) -> WormCellReader<'a, T> {
        WormCellReader::<'a, T> { value: unsafe { &*self.value.get() } }
    }
}

impl<'a, T> WormCellReader<'a, T> {
    fn get(&self) -> WormCellResult<&'a T> {
        match self.value {
            Some(ref val) => Ok(val),
            None => Err(WormCellError::ReadNotSet)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn make_one() {
        let worm = WormCell::<i32>::new();
        let reader = worm.reader();
                
        worm.set(5);

        assert_eq!(*reader.get().unwrap(), 5);
    }
}

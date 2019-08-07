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
    value: Box<UnsafeCell<Option<T>>>
}

pub struct WormCellReader<'a, T: Sized> {
    value: &'a Option<T>
}

impl<'a, T> WormCell<T> {
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

    pub fn reader(&self) -> WormCellReader<'a, T> {
        WormCellReader::<'a, T> { value: unsafe { &*self.value.get() } }
    }
}

impl<'a, T> WormCellReader<'a, T> {
    pub fn get(&self) -> WormCellResult<&'a T> {
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
    fn one_reader() {
        let worm = WormCell::<i32>::new();
        let reader = worm.reader();
                
        worm.set(5).unwrap();

        assert_eq!(*reader.get().unwrap(), 5);
    }

    #[test]
    fn multi_reader() {
        let worm = WormCell::<i32>::new();
        let reader1 = worm.reader();
        let reader2 = worm.reader();
        let reader3 = worm.reader();
                
        worm.set(5).unwrap();

        assert_eq!(*reader1.get().unwrap(), 5);
        assert_eq!(*reader2.get().unwrap(), 5);
        assert_eq!(*reader3.get().unwrap(), 5);
    }

    struct HasCell {
        c: WormCell<i32>
    }

    impl HasCell {
        fn set(&self, val: i32) {
            self.c.set(val).unwrap();
        }
    }
    
    #[test]
    fn move_cell() {
        let worm = WormCell::<i32>::new();
        let reader1 = worm.reader();
        let reader2 = worm.reader();
        let reader3 = worm.reader();

        let hc = HasCell{c: worm};
        
        hc.set(5);
        
        assert_eq!(*reader1.get().unwrap(), 5);
        assert_eq!(*reader2.get().unwrap(), 5);
        assert_eq!(*reader3.get().unwrap(), 5);
    }

    struct HasReader<'a> {
        wr: WormCellReader<'a, i32>
    }
    
    #[test]
    fn move_reader() {
        let worm = WormCell::<i32>::new();
        let reader1 = worm.reader();
        let reader2 = worm.reader();
        let reader3 = worm.reader();

        worm.set(5).unwrap();

        let hr = HasReader { wr: reader1 };
        
        assert_eq!(*hr.wr.get().unwrap(), 5);
        assert_eq!(*reader2.get().unwrap(), 5);
        assert_eq!(*reader3.get().unwrap(), 5);
    }
    
}

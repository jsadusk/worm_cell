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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_reader() {
        let worm = WormCell::<i32>::new();
        assert!(!worm.is_set());
        let reader = worm.reader();
        assert!(!worm.is_set());
                
        worm.set(5).unwrap();
        assert!(worm.is_set());

        assert_eq!(*reader.get().unwrap(), 5);
    }

    #[test]
    fn multi_reader() {
        let worm = WormCell::<i32>::new();
        assert!(!worm.is_set());
        let reader1 = worm.reader();
        assert!(!worm.is_set());
        let reader2 = worm.reader();
        assert!(!worm.is_set());
        let reader3 = worm.reader();
        assert!(!worm.is_set());
                
        worm.set(5).unwrap();
        assert!(worm.is_set());

        assert_eq!(*reader1.get().unwrap(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader2.get().unwrap(), 5);
        assert!(worm.is_set());
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
        assert!(!worm.is_set());
        let reader1 = worm.reader();
        assert!(!worm.is_set());
        let reader2 = worm.reader();
        assert!(!worm.is_set());
        let reader3 = worm.reader();

        assert!(!worm.is_set());
        let hc = HasCell{c: worm};
        
        assert!(!hc.c.is_set());
        hc.set(5);
        
        assert!(hc.c.is_set());
        assert_eq!(*reader1.get().unwrap(), 5);
        assert!(hc.c.is_set());
        assert_eq!(*reader2.get().unwrap(), 5);
        assert!(hc.c.is_set());
        assert_eq!(*reader3.get().unwrap(), 5);
    }

    struct HasReader<'a> {
        wr: WormCellReader<'a, i32>
    }
    
    #[test]
    fn move_reader() {
        let worm = WormCell::<i32>::new();
        assert!(!worm.is_set());
        let reader1 = worm.reader();
        assert!(!worm.is_set());
        let reader2 = worm.reader();
        assert!(!worm.is_set());
        let reader3 = worm.reader();

        assert!(!worm.is_set());
        worm.set(5).unwrap();

        assert!(worm.is_set());
        let hr = HasReader { wr: reader1 };
        
        assert!(worm.is_set());
        assert_eq!(*hr.wr.get().unwrap(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader2.get().unwrap(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader3.get().unwrap(), 5);
    }
    
}

#[cfg(test)]
mod test {
    use crate::worm_cell::*;

    struct HasCell {
        c: WormCell<i32>
    }

    impl HasCell {
        fn set(&self, val: i32) {
            self.c.set(val).unwrap();
        }
    }


    struct HasReader {
        wr: WormCellReader<i32>
    }

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

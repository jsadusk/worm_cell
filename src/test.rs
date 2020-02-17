#[cfg(test)]
mod test {
    use crate::worm_cell::*;
    use std::rc::Rc;
    use std::sync::Arc;

    struct HasCell {
        c: Rc<WormCell<i32>>
    }

    impl HasCell {
        fn set(&self, val: i32) {
            self.c.set(val);
        }
    }


    struct HasReader {
        wr: WormCellReader<i32>
    }

    #[test]
    fn one_reader() {
        let worm = Rc::new(WormCell::<i32>::new());
        assert!(!worm.is_set());
        let reader = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        
        worm.set(5);
        assert!(worm.is_set());

        assert_eq!(*reader.get(), 5);
    }

    #[test]
    fn multi_reader() {
        let worm = Rc::new(WormCell::<i32>::new());
        assert!(!worm.is_set());
        let reader1 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader2 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader3 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        
        worm.set(5);
        assert!(worm.is_set());

        assert_eq!(*reader1.get(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader2.get(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader3.get(), 5);
    }

    #[test]
    fn move_cell() {
        let worm = Rc::new(WormCell::<i32>::new());
        assert!(!worm.is_set());
        let reader1 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader2 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader3 = WormCellReader::new(worm.clone());

        assert!(!worm.is_set());
        let hc = HasCell{c: worm};
        
        assert!(!hc.c.is_set());
        hc.set(5);
        
        assert!(hc.c.is_set());
        assert_eq!(*reader1.get(), 5);
        assert!(hc.c.is_set());
        assert_eq!(*reader2.get(), 5);
        assert!(hc.c.is_set());
        assert_eq!(*reader3.get(), 5);
    }

    #[test]
    fn move_reader() {
        let worm = Rc::new(WormCell::<i32>::new());
        assert!(!worm.is_set());
        let reader1 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader2 = WormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        let reader3 = WormCellReader::new(worm.clone());

        assert!(!worm.is_set());
        worm.set(5);

        assert!(worm.is_set());
        let hr = HasReader { wr: reader1 };
        
        assert!(worm.is_set());
        assert_eq!(*hr.wr.get(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader2.get(), 5);
        assert!(worm.is_set());
        assert_eq!(*reader3.get(), 5);
    }

    #[test]
    fn atomic_one_reader() {
        let worm = Arc::new(AtomicWormCell::<i32>::new());
        assert!(!worm.is_set());
        let reader = AtomicWormCellReader::new(worm.clone());
        assert!(!worm.is_set());
        
        worm.set(5);
        assert!(worm.is_set());

        assert_eq!(*reader.get(), 5);
    }
}

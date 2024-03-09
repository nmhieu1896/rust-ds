use std::cell::UnsafeCell;

#[allow(dead_code)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// impl<T> !Sync for Cell<T> {}

#[allow(dead_code)]
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn get_clone(&self) -> T
    where
        T: Clone,
    {
        unsafe { (*self.value.get()).clone() }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }
}

#[cfg(test)]
mod test_cell {
    use super::Cell;

    #[test]
    fn test1() {
        let x = Cell::new(10);
        assert_eq!(x.get(), 10);
        x.set(15);
        assert_eq!(x.get(), 15);

        let y = Cell::new(vec![10]);
        assert_eq!(y.get_clone(), vec![10]);
        y.set(vec![10, 15, 20]);
        assert_eq!(y.get_clone(), vec![10, 15, 20]);
    }
}

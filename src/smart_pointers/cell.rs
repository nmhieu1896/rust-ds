use std::cell::UnsafeCell;

#[allow(dead_code)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

#[allow(dead_code)]
impl<T: Copy> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
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
        assert_eq!(x.get(), 10)
    }
}

use super::cell::Cell;
use std::cell::UnsafeCell;

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum ShareType {
    Unshared,
    Shared(usize),
    Exclusive,
}

#[allow(dead_code)]
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<ShareType>,
}

#[allow(dead_code)]
impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(ShareType::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<&T> {
        match self.state.get() {
            ShareType::Exclusive => None,
            ShareType::Unshared => {
                self.state.set(ShareType::Shared(1));
                Some(unsafe { &*self.value.get() })
            }
            ShareType::Shared(n) => {
                self.state.set(ShareType::Shared(n + 1));
                Some(unsafe { &*self.value.get() })
            }
        }
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        match self.state.get() {
            ShareType::Shared(_) | ShareType::Exclusive => None,
            ShareType::Unshared => Some(unsafe { &mut *self.value.get() }),
        }
    }
}

#[cfg(test)]
mod test_refcell {
    use super::RefCell;

    #[test]
    pub fn test_borrow_refcell() {
        let x = RefCell::new(10);
        assert_eq!(x.borrow().unwrap(), &10);
    }
    #[test]
    pub fn test_multiple_borrow() {
        let x = RefCell::new(10);
        let y = x.borrow();
        let z = x.borrow();
        assert_eq!(z, y);
    }

    #[test]
    pub fn test_mut_borrow() {
        let x = RefCell::new(10);
        let y = x.borrow_mut().unwrap();
        let z = x.borrow_mut().unwrap();
        assert_eq!(z, y);
        *y = 15;
        assert_eq!(x.borrow().unwrap(), &15)
    }
}

use super::cell::Cell;
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

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

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

#[allow(dead_code)]
impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(ShareType::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<T>> {
        match self.state.get() {
            ShareType::Exclusive => None,
            ShareType::Unshared => {
                self.state.set(ShareType::Shared(1));
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
            ShareType::Shared(n) => {
                self.state.set(ShareType::Shared(n + 1));
                // Some(unsafe { &*self.value.get() })
                Some(Ref { refcell: self })
            }
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<T>> {
        match self.state.get() {
            ShareType::Shared(_) | ShareType::Exclusive => None,
            ShareType::Unshared => {
                self.state.set(ShareType::Exclusive);
                // Some(unsafe { &mut *self.value.get() })
                Some(RefMut { refcell: self })
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}
impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}
impl<T> DerefMut for RefMut<'_, T> {
    // type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            ShareType::Shared(_) | ShareType::Unshared => unreachable!(),
            ShareType::Exclusive => self.refcell.state.set(ShareType::Unshared),
        }
    }
}
impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            ShareType::Exclusive | ShareType::Unshared => unreachable!(),
            ShareType::Shared(1) => self.refcell.state.set(ShareType::Unshared),
            ShareType::Shared(n) => self.refcell.state.set(ShareType::Shared(n - 1)),
        }
    }
}

#[cfg(test)]
mod test_refcell {
    use super::RefCell;

    #[test]
    pub fn test_borrow_refcell() {
        let x = RefCell::new(10);
        assert_eq!(*x.borrow().unwrap(), 10);
    }
    #[test]
    pub fn test_multiple_borrow() {
        let x = RefCell::new(10);
        let y = x.borrow();
        let z = x.borrow();
        assert_eq!(*z.unwrap(), *y.unwrap());
    }

    #[test]
    pub fn test_mut_borrow() {
        let x = RefCell::new(10);
        let y = x.borrow_mut().unwrap();

        assert_eq!(*y, 10);
        unsafe { *y.refcell.value.get() = 15 };
        // Cann not borrow exclusive share
        let z = x.borrow_mut();
        assert!(z.is_none());
        let z = x.borrow();
        assert!(z.is_none());
        drop(y);
        //After drop, it is sharable
        let z = *x.borrow_mut().unwrap();
        assert_eq!(z, 15);
    }
}

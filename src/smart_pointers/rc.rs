use super::cell::Cell;
use std::ops::Deref;

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

#[allow(dead_code)]
pub struct Rc<T> {
    inner: *const RcInner<T>,
}

#[allow(dead_code)]
impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: Cell::new(1),
        });
        Rc {
            inner: Box::into_raw(inner),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        let c = inner.ref_count.get();
        inner.ref_count.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner };
        let c = inner.ref_count.get();
        if c == 1 {
            drop(inner);
            drop(self.inner)
            // let _ = Box::from_raw(self.inner);
        } else {
            inner.ref_count.set(c - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let rc = Rc::new(42);
        let rc2 = rc.clone();
        assert_eq!(*rc, 42);
        assert_eq!(*rc2, 42);
    }
}

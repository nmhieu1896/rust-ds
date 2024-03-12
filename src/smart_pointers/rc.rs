use std::ops::Deref;

struct RcInner<T> {
    value: T,
    ref_count: usize,
}

#[allow(dead_code)]
pub struct Rc<T> {
    inner: *const RcInner<T>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: 1,
        });
        Rc {
            inner: Box::into_raw(inner),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { &*self.inner };
        inner.ref_count += 1;
        Rc { inner: self.inner }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

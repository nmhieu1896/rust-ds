pub mod mut_pointers;
use std::{
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping Mybox data !");
    }
}

impl<T> fmt::Pointer for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:p}", self)
    }
}

#[allow(dead_code)]
pub fn _run() {
    let x = 5;
    let y = &x;
    let y1 = &y;
    let z = Box::new(x);
    let w = MyBox::new(x);
    let msg = MyBox::new(String::from("Rust"));

    println!("{:p}", &x);
    println!("{:p}", y);
    println!("{:p}", w);
    println!("{:p}", y1);
    println!("{:p}", z);
    hello(&msg);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod test_pointers {
    use super::*;
    #[test]
    fn basic_pointers() {
        let x = 5;
        let mut mut_x = 5;
        let ref_x = &x;
        let box_x = Box::new(x);
        let mybox_x = MyBox::new(x);
        let mut mut_mybox_x = MyBox::new(mut_x);

        assert_eq!(5, x);
        assert_eq!(5, *ref_x);
        assert_eq!(box_x.as_ref(), box_x.deref());
        assert_eq!(*box_x, *ref_x);
        assert_eq!(*box_x, *mybox_x);
        assert_eq!(&5, mybox_x.deref());
        assert_eq!(*box_x, *mybox_x.deref());
        assert_eq!(ref_x, mybox_x.deref());
        assert_eq!(&mut 5, mut_mybox_x.deref_mut());
        assert_eq!(&mut_x, mut_mybox_x.deref_mut());
        assert_eq!(mybox_x.deref(), mut_mybox_x.deref_mut());
    }
}

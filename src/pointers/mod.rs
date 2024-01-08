use std::{fmt, ops::Deref};

#[derive(Debug)]
struct MyBox<T>(T);

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

impl<T> fmt::Pointer for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:p}", self)
    }
}

pub fn _run() {
    let x = 5;
    let y = &x;
    let y1 = &y;
    let z = Box::new(x);
    let w = MyBox::new(x);
    let msg = MyBox::new(String::from("Rust"));
    let msg_ref = &msg;

    println!("{:p}", y);
    println!("{:p}", w);
    println!("{:p}", y1);
    println!("{:p}", z);
    hello(&msg)
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
        let ref_x = &x;
        let box_x = Box::new(x);
        let mybox_x = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *ref_x);
        assert_eq!(*box_x, *ref_x);
        assert_eq!(*box_x, *mybox_x);
        assert_eq!(&5, mybox_x.deref());
        assert_eq!(*box_x, *mybox_x.deref());
        assert_eq!(ref_x, mybox_x.deref());
    }
}

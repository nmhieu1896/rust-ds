use std::{fmt, rc::Rc};

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        let x = self.head.as_deref();
        x.map(|v| &v.elem)
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            // head: self.head.as_ref().map(|node| node.next.clone()),
        }
    }
}

pub fn _run() {
    let list = List::new();
    let list = list.prepend(1).prepend(2).prepend(3);
    println!("{:?}", list);
    println!("{:?}", list.head());
    let list = list.tail();
    println!("{:?}", list);
}

// -- debug
impl<T> fmt::Debug for List<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = "".to_string();
        match self.head {
            None => {
                fmt_str = "Empty".to_string();
            }
            Some(ref node) => {
                fmt_str.push_str(&format!("Node({:?})", node.elem));
                let mut next = &node.next;
                loop {
                    match next {
                        None => break,
                        Some(ref node) => {
                            fmt_str.push_str(&format!("->Node({:?})", node.elem));
                            next = &node.next;
                        }
                    }
                }
            }
        }
        write!(f, "{}", fmt_str)
    }
}

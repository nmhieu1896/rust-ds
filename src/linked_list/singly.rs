use std::fmt;
use std::mem;

struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    // Self impl to avoid blowing the stack
    // by recursively call nested drop of list of Box(Node)
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, None);
        while let Some(mut box_node) = curr_link {
            curr_link = mem::replace(&mut box_node.next, None)
            // box_node goes out of scope and gets dropped
        }
    }
}

impl fmt::Debug for List {
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

pub fn _run() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(5);
    list.push(3);
    println!("{:?}", list);
    list.pop();
    println!("{:?}", list);
}

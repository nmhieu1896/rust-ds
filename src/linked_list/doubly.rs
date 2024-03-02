use std::cell::{Ref, RefCell};
use std::fmt::Debug;
// use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node)
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head)
                }
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub fn _run() {
    let mut list = List::<i32>::new();
    list.push_front(1);
    println!("{:?}", list);
    list.push_front(2);
    list.push_front(5);
    list.push_front(3);
    println!("{:?}", list.peek());
    let pop_value = list.pop_front();
    println!("{:?}", pop_value);
    let pop_value = list.pop_front();
    println!("{:?}", pop_value);
    let pop_value = list.pop_front();
    println!("{:?}", pop_value);
    let pop_value = list.pop_front();
    println!("{:?}", pop_value);
    println!("{:?}", list);
}

// -- debug
// impl<T> fmt::Debug for List<T>
// where
//     T: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut fmt_str = "".to_string();
//         match self.head {
//             None => {
//                 fmt_str = "Empty".to_string();
//             }
//             Some(ref node) => {
//                 fmt_str.push_str(&format!("Node({:?})", node.clone().borrow_mut().elem));
//                 // let next = node.borrow_mut().next.clone();
//                 // implement debug so that it will print
//                 // Node(1) <-> Node(2) <-> Node(5) <-> Node(3)
//                 // let mut next = node.borrow_mut();

//                 let mut next = Rc::clone(node);
//                 loop {
//                     let current = next.borrow();
//                     match current.next {
//                         None => break,
//                         Some(ref node) => {
//                             fmt_str.push_str(&format!("<->Node({:?})", node.borrow().elem));
//                             // gg = node.clone();
//                             // next = node.borrow_mut();
//                         }
//                     }
//                     let abc = Rc::clone(&next);
//                     let xyz = &abc.borrow().next;
//                     if let Some(gg) = xyz {
//                         next = Rc::clone(gg);
//                     } else {
//                         break;
//                     };
//                 }
//             }
//         }
//         write!(f, "{}", fmt_str)
//     }
// }

use std::fmt;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    // Self impl to avoid blowing the stack
    // by recursively call nested drop of list of Box(Node)
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut box_node) = curr_link {
            curr_link = box_node.next.take()
            // box_node goes out of scope and gets dropped
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| &*node);
            &node.elem
        })
    }
}

pub fn _run() {
    let mut list = List::<i32>::new();
    list.push(1);
    list.push(2);
    list.push(5);
    list.push(3);
    println!("{:?}", list);
    list.pop();
    println!("{:?}", list);
    let head = list.peek();
    println!("Head: {:?}", head);
    list.peek_mut().map(|node| {
        *node *= 4;
    });
    println!("List After mut: {:?}", list);

    let mut idx = 0;
    list.iter().for_each(|e| {
        idx += 1;
        println!("item_iter {}: {:?}", idx, e)
    });
    list.into_iter().for_each(|e| {
        idx += 1;
        println!("item {}: {:?}", idx, e)
    });
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

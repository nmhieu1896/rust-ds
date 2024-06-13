use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryTree {
    root: Option<TreeNodeRef>,
}

#[allow(dead_code)]
impl BinaryTree {
    pub fn new(val: i32) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            }))),
        }
    }

    pub fn insert(&mut self, val: Option<i32>) {
        //insert from left to right, first come first serve, dont care about value
        let mut vec: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let curr = Rc::clone(self.root.as_mut().unwrap());
        vec.push_back(curr);
        loop {
            let curr = vec.pop_front();
            if curr.is_none() {
                continue;
            }
            let curr = curr.unwrap();
            if curr.borrow().left.is_some() && curr.borrow().right.is_some() {
                let left = Rc::clone(curr.borrow_mut().left.as_mut().unwrap());
                let right = Rc::clone(curr.borrow_mut().right.as_mut().unwrap());
                vec.push_back(left);
                vec.push_back(right);
                continue;
            }
            let new_node = if val.is_some() {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: val.unwrap(),
                    left: None,
                    right: None,
                })))
            } else {
                None
            };

            if curr.borrow().left.is_none() {
                curr.borrow_mut().left = new_node;
            } else {
                curr.borrow_mut().right = new_node;
            }
            break;
        }
    }

    //depth first search
    // pub fn traverse_in_order(node: Option<&TreeNodeRef>) {
    //     if let Some(node) = node {
    //         TreeNode::traverse_in_order(node.borrow().left.as_ref());
    //         println!("{}", node.borrow().val);
    //         TreeNode::traverse_in_order(node.borrow().right.as_ref());
    //     }
    // }
    //breadth first search
    pub fn traverse_level_order(node: Option<TreeNodeRef>) {
        let mut queue = std::collections::VecDeque::new();
        if let Some(node) = node {
            queue.push_back(node);
        }
        while let Some(node) = queue.pop_front() {
            println!("{}", node.borrow().val);
            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
            }
        }
    }
}

pub fn _run() {}

#[cfg(test)]
mod tests_binary_tree {
    use super::*;

    #[test]
    fn it_works() {
        let mut root = BinaryTree::new(1);
        root.insert(Some(2));
        root.insert(Some(3));
        root.insert(Some(4));
        root.insert(Some(5));
        root.insert(Some(6));
        root.insert(Some(7));
        println!("{:#?}", root);

        // TreeNode::traverse_in_order(Some(&Rc::new(RefCell::new(root.clone()))));
        // println!("====================");
        // TreeNode::traverse_level_order(Some(Rc::new(RefCell::new(root.clone()))));
        assert!(false)
    }
}

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: i32) {
        if val < self.val {
            if let Some(left) = &mut self.left {
                left.borrow_mut().insert(val);
            } else {
                self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.borrow_mut().insert(val);
            } else {
                self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        }
    }

    //depth first search
    fn traverse_in_order(node: Option<&TreeNodeRef>) {
        if let Some(node) = node {
            TreeNode::traverse_in_order(node.borrow().left.as_ref());
            println!("{}", node.borrow().val);
            TreeNode::traverse_in_order(node.borrow().right.as_ref());
        }
    }
    //breadth first search
    fn traverse_level_order(node: Option<TreeNodeRef>) {
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

pub fn _run() {
    let mut root = TreeNode::new(5);
    root.insert(3);
    root.insert(7);
    root.insert(2);
    root.insert(4);
    root.insert(6);
    root.insert(8);
    println!("{:#?}", root);

    TreeNode::traverse_in_order(Some(&Rc::new(RefCell::new(root.clone()))));
    println!("====================");
    TreeNode::traverse_level_order(Some(Rc::new(RefCell::new(root.clone()))));
}

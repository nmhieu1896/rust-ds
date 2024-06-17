use std::cell::RefCell;
use std::rc::Rc;

use super::binary_tree::TreeNode;

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::mirror(&root, &root);
    }
    fn val_of(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return node.as_deref().unwrap().borrow().val;
    }
    fn mirror(n1: &Option<Rc<RefCell<TreeNode>>>, n2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if n1.is_none() && n2.is_none() {
            return true;
        }
        if n1.is_none() || n2.is_none() {
            return false;
        }
        return Self::val_of(&n1) == Self::val_of(&n2)
            && Self::mirror(
                &n1.as_ref().unwrap().borrow().left,
                &n2.as_ref().unwrap().borrow().right,
            )
            && Self::mirror(
                &n1.as_ref().unwrap().borrow().right,
                &n2.as_ref().unwrap().borrow().left,
            );
    }
}

#[cfg(test)]
mod tests_symmetric_tree {
    use super::*;
    #[test]
    fn test1() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.left.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(root)))));
    }
}

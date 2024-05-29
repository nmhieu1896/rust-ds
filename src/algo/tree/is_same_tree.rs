use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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
struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // p==q is enough in rust =))
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        if p.as_ref().unwrap().borrow().val != q.as_ref().unwrap().borrow().val {
            return false;
        }
        let left = p.as_ref().unwrap().borrow().left.clone();
        let right = p.as_ref().unwrap().borrow().right.clone();
        let left2 = q.as_ref().unwrap().borrow().left.clone();
        let right2 = q.as_ref().unwrap().borrow().right.clone();
        Self::is_same_tree(left, left2) && Self::is_same_tree(right, right2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::is_same_tree(p, q), true);
    }

    #[test]
    fn test_is_same_tree2() {
        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}

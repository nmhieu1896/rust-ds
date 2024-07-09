use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
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
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }

    pub fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);
        let mid = inorder.iter().position(|&x| x == preorder[0]).unwrap();
        root.left = Self::build(&preorder[1..mid + 1], &inorder[..mid]);
        root.right = Self::build(&preorder[mid + 1..], &inorder[mid + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}

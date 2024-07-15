// Definition for a binary tree node.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut root: HashSet<i32> = HashSet::new();
        let mut non_root: HashSet<i32> = HashSet::new();

        for des in descriptions {
            let parent_val = des[0];
            let child_val = des[1];
            let is_left = des[2] == 1;

            let parent = map
                .entry(parent_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent_val))))
                .clone();

            if !non_root.contains(&parent.as_ref().borrow().val) {
                root.insert(parent.as_ref().borrow().val);
            }

            let child = map
                .entry(child_val)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child_val))))
                .clone();

            non_root.insert(child.as_ref().borrow().val);
            root.remove(&child.as_ref().borrow().val);

            if is_left {
                parent.borrow_mut().left = Some(child);
            } else {
                parent.borrow_mut().right = Some(child);
            }
        }

        let key = root.into_iter().next().unwrap();
        map.get(&key).map(|x| x.clone())
    }
}

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

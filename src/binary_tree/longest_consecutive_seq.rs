use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let val = root.as_ref().map(|x| x.borrow().val).unwrap();
        Self::recursive(root.as_ref(), val, 0)
    }
    pub fn recursive(root: Option<&Rc<RefCell<TreeNode>>>, parent: i32, mut count: i32) -> i32 {
        if root.is_none() {
            return count;
        }
        let root = root.unwrap();
        count = if root.borrow().val - parent == 1 {
            count + 1
        } else {
            1
        };
        return count
            .max(Self::recursive(
                root.borrow().left.as_ref(),
                root.borrow().val,
                count,
            ))
            .max(Self::recursive(
                root.borrow().right.as_ref(),
                root.borrow().val,
                count,
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(
            Solution::longest_consecutive(Some(Rc::new(RefCell::new(root)))),
            3
        );
    }

    #[test]
    fn it_works2() {
        let mut root = TreeNode::new(1);
        root.insert(3);
        root.insert(2);
        root.insert(4);
        root.insert(5);
        assert_eq!(
            Solution::longest_consecutive(Some(Rc::new(RefCell::new(root)))),
            3
        );
    }
    #[test]
    fn it_works3() {
        let mut root = TreeNode::new(2);
        root.insert(3);
        root.insert(2);
        root.insert(1);
        assert_eq!(
            Solution::longest_consecutive(Some(Rc::new(RefCell::new(root)))),
            2
        );
    }
}

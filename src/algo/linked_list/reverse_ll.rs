// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;
        while current.is_some() {
            let next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = prev;
            prev = current;
            current = next;
        }
        return prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut list = ListNode::new(2);
        list.next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next = Some(Box::new(list));
        let mut list2 = ListNode::new(4);
        list2.next = Some(Box::new(ListNode::new(5)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(list2));
        println!("HEAD:{:?}", head);
        let reverse_ll = Solution::reverse_list(head);
        println!("REV: {:?}", reverse_ll);
        let reverse_ll = reverse_ll.unwrap();
        assert_eq!(reverse_ll.val, 5);
        let next = reverse_ll.next.unwrap();
        assert_eq!(next.val, 4);
        let next = next.next.unwrap();
        assert_eq!(next.val, 3,);
        let next = next.next.unwrap();
        assert_eq!(next.val, 2,);
        let next = next.next.unwrap();
        assert_eq!(next.val, 1,);
    }
}

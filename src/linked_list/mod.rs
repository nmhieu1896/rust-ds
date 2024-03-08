pub mod doubly;
pub mod rc_singly;
pub mod singly;

#[cfg(test)]
mod test_singly {
    use super::singly::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|node| {
            *node *= 4;
        });
        assert_eq!(list.peek_mut(), Some(&mut 12));
        assert_eq!(list.peek(), Some(&12));
        assert_eq!(list.pop(), Some(12));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        // let mut iter = list.iter_mut();
        // assert_eq!(iter.next(), Some(&mut 3));
        // assert_eq!(iter.next(), Some(&mut 2));
        // assert_eq!(iter.next(), Some(&mut 1));
        // assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod test_rc_singly {
    use super::rc_singly::List;

    #[test]
    fn basics() {
        let list = List::new();
        let list = list.prepend(1).prepend(2).prepend(3);
        println!("{:?}", list);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        println!("{:?}", list);
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}

#[cfg(test)]
mod test_doubly_list {
    use super::doubly;

    #[test]
    fn test_push_pop() {
        let mut list = doubly::List::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        //test peak and pop front
        assert_eq!(*list.peek().unwrap(), 3);
        list.pop_front();
        assert_eq!(*list.peek().unwrap(), 2);
        list.pop_front();
        assert_eq!(*list.peek().unwrap(), 1);
        //test tail and pop back
        assert_eq!(*list.tail().unwrap(), 6);
        list.pop_back();
        assert_eq!(*list.tail().unwrap(), 5);
        list.pop_back();
        assert_eq!(*list.tail().unwrap(), 4);
    }

    #[test]
    fn test_iter() {
        let mut list = doubly::List::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        let mut iter = list.into_iter();
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 1);
        assert_eq!(iter.next().unwrap(), 4);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next().unwrap(), 6);
    }
}

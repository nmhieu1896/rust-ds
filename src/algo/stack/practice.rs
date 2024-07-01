use std::collections::VecDeque;

#[allow(dead_code)]
struct RecentCounter {
    pub deque: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            deque: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.deque.push_back(t);
        while self.deque[0] < t - 3000 {
            self.deque.pop_front();
        }
        self.deque.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let mut obj = RecentCounter::new();
        assert_eq!(obj.ping(1), 1);
        assert_eq!(obj.ping(100), 2);
        assert_eq!(obj.ping(3001), 3);
        assert_eq!(obj.ping(3002), 3);
    }
}

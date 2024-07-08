use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut q: VecDeque<i32> = (1..=n).collect();

        while q.len() > 1 {
            for _ in 0..k - 1 {
                if let Some(front) = q.pop_front() {
                    q.push_back(front);
                }
            }
            q.pop_front();
        }

        q.pop_front().unwrap()
    }
}

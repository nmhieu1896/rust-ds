use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut in_degrees = vec![0; num_courses as usize];
        let mut adj = vec![vec![]; num_courses as usize];

        for item in prerequisites.iter() {
            in_degrees[item[0] as usize] += 1;
            adj[item[1] as usize].push(item[0]);
        }
        let mut res = vec![];
        let mut q = VecDeque::new();

        for (c, in_deg) in in_degrees.iter().enumerate() {
            if *in_deg == 0 {
                q.push_back(c as i32);
            }
        }

        while q.len() > 0 {
            let course = q.pop_front().unwrap();
            res.push(course);
            for next_c in adj[course as usize].iter() {
                in_degrees[*next_c as usize] -= 1;
                if in_degrees[*next_c as usize] == 0 {
                    q.push_back(*next_c)
                }
            }
        }
        if in_degrees.iter().any(|&x| x != 0) {
            return vec![];
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let expected = vec![0, 1, 2, 3];
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }
}

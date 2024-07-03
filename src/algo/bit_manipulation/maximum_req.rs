#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let m = requests.len();
        let mut res = 0;

        for mask in 0..((1 << m) as u32) {
            if Self::check(mask as i32, &requests, n) {
                res = res.max(mask.count_ones())
            }
        }
        return res as i32;
    }

    pub fn check(mask: i32, requests: &Vec<Vec<i32>>, n: i32) -> bool {
        let m = requests.len();
        let mut degree = vec![0; n as usize];
        for i in 0..m {
            if Self::is_bit1(mask, i as i32) {
                degree[requests[i][0] as usize] -= 1;
                degree[requests[i][1] as usize] += 1;
            }
        }

        return degree.iter().all(|&x| x == 0);
    }

    pub fn is_bit1(mask: i32, nth_bit: i32) -> bool {
        return (mask >> nth_bit & 1) == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let requests = vec![
            vec![0, 1],
            vec![1, 0],
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![3, 4],
        ];
        assert_eq!(Solution::maximum_requests(n, requests), 5);
    }

    #[test]
    fn test2() {
        let n = 3;
        let requests = vec![vec![0, 0], vec![1, 2], vec![2, 1]];
        assert_eq!(Solution::maximum_requests(n, requests), 3);
    }
}

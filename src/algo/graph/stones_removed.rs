#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn share_pos(stone1: &Vec<i32>, stone2: &Vec<i32>) -> bool {
        return stone1[0] == stone2[0] || stone1[1] == stone2[1];
    }

    pub fn dfs(idx: usize, stones: &Vec<Vec<i32>>, visit: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
        visit[idx] = true;
        for i in adj[idx].iter() {
            if visit[*i as usize] {
                continue;
            }
            Self::dfs(*i as usize, stones, visit, adj);
        }
    }

    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut visit = vec![false; stones.len()];
        let mut adj = vec![vec![]; stones.len()];

        for i in 0..stones.len() {
            for j in i + 1..stones.len() {
                if Self::share_pos(&stones[i], &stones[j]) {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        let mut count = 0;
        for i in 0..stones.len() {
            if visit[i] {
                continue;
            }
            count += 1;
            Self::dfs(i, &stones, &mut visit, &adj);
        }

        return stones.len() as i32 - count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let stones = vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2],
        ];
        assert_eq!(Solution::remove_stones(stones), 5);
    }

    #[test]
    fn test2() {
        let stones = vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]];
        assert_eq!(Solution::remove_stones(stones), 3);
    }

    #[test]
    fn test3() {
        let stones = vec![vec![0, 0]];
        assert_eq!(Solution::remove_stones(stones), 0);
    }
}

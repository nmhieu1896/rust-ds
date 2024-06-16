#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut vis = vec![false; is_connected.len()];
        let mut provinces = 0;
        for i in 0..is_connected.len() {
            if vis[i] {
                continue;
            }
            provinces += 1;
            Self::dfs(i, &is_connected, &mut vis);
        }

        return provinces;
    }
    pub fn dfs(curr: usize, is_connected: &Vec<Vec<i32>>, vis: &mut Vec<bool>) {
        vis[curr] = true;
        for (idx, &connected) in is_connected[curr].iter().enumerate() {
            if vis[idx] {
                continue;
            }
            if connected == 1 {
                Self::dfs(idx, is_connected, vis);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 1, 0]];
        assert_eq!(Solution::find_circle_num(is_connected), 2);
    }

    #[test]
    fn test2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(is_connected), 3);
    }
}

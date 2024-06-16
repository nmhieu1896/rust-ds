struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut vis = vec![false; n];
        Self::dfs(0, &rooms, &mut vis);

        return vis.iter().all(|&x| x);
    }
    fn dfs(idx: usize, rooms: &Vec<Vec<i32>>, vis: &mut Vec<bool>) {
        vis[idx] = true;
        for room in rooms[idx].iter() {
            if *vis.get(*room as usize).unwrap() {
                continue;
            }
            Self::dfs(*room as usize, rooms, vis);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let rooms = vec![vec![1], vec![2], vec![3], vec![]];
        assert_eq!(Solution::can_visit_all_rooms(rooms), true);
    }

    #[test]
    fn test2() {
        let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        assert_eq!(Solution::can_visit_all_rooms(rooms), false);
    }
}

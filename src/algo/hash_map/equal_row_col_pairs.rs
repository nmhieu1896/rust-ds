use std::collections::HashMap;
#[allow(dead_code)]
pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    for v in grid.iter() {
        *map.entry(v).or_insert(0) += 1;
    }

    for i in 0..grid.len() {
        let mut v = Vec::with_capacity(grid.len());
        for j in 0..grid.len() {
            v.push(grid[j][i]);
        }
        if let Some(inc) = map.get(&v) {
            count += inc;
        }
    }

    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal_pairs1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        assert_eq!(equal_pairs(grid), 1);
    }

    #[test]
    fn test_equal_pairs2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        assert_eq!(equal_pairs(grid), 3);
    }
    #[test]
    fn test_equal_pairs3() {
        let grid = vec![vec![1, 1], vec![1, 1]];
        assert_eq!(equal_pairs(grid), 4);
    }
    #[test]
    fn test_equal_pairs4() {
        let grid = vec![vec![11, 1], vec![1, 11]];
        assert_eq!(equal_pairs(grid), 2);
    }
}

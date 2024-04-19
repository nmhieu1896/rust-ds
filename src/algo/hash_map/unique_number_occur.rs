use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    for &num in arr.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut set = HashSet::new();
    for &val in map.values() {
        if !set.insert(val) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_occurrences() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert_eq!(unique_occurrences(arr), true);

        let arr = vec![1, 2];
        assert_eq!(unique_occurrences(arr), false);

        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert_eq!(unique_occurrences(arr), true);
    }
}

use std::collections::HashSet;

#[allow(dead_code)]
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let set: HashSet<_> = nums.iter().collect();
    let mut longest = 1;
    for &&i in set.iter() {
        if set.contains(&(i - 1)) {
            continue;
        }
        let mut current = 1;
        while set.contains(&(i + current)) {
            current += 1;
            longest = longest.max(current);
        }
    }
    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_consecutive1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_longest_consecutive2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }
}

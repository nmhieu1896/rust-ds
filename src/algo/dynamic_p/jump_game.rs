#[allow(dead_code)]
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_reachable = 0;
    for (i, &num) in nums.iter().enumerate() {
        if i > max_reachable {
            return false;
        }
        max_reachable = max_reachable.max(i + num as usize);
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(can_jump(nums), true);
    }

    #[test]
    fn test_can_jump2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(can_jump(nums), false);
    }
    #[test]
    fn test_can_jump3() {
        let nums = vec![4, 0, 0, 0, 0];
        assert_eq!(can_jump(nums), true);
    }
}

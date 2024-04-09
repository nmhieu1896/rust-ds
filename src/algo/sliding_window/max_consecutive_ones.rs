#[allow(dead_code)]
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut count = 0;
    nums.iter().enumerate().for_each(|(right, n)| {
        if n == &0 {
            count += 1;
            while left < right && count == 2 {
                if nums[left] == 0 {
                    count -= 1;
                }
                left += 1;
            }
        }
        max = max.max(right - left + 1);
    });
    return max as i32;
}

#[cfg(test)]
mod tests {
    use super::find_max_consecutive_ones;

    #[test]
    fn test1() {
        let nums = vec![1, 0, 1, 1, 0];
        let res = find_max_consecutive_ones(nums);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let res = find_max_consecutive_ones(nums);
        assert_eq!(res, 4);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 1, 1, 1];
        let res = find_max_consecutive_ones(nums);
        assert_eq!(res, 6);
    }

    #[test]
    fn test4() {
        let nums = vec![0, 0, 0, 0, 0, 0];
        let res = find_max_consecutive_ones(nums);
        assert_eq!(res, 1);
    }

    #[test]
    fn test5() {
        let nums = vec![1, 1, 1, 0, 0, 0];
        let res = find_max_consecutive_ones(nums);
        assert_eq!(res, 4);
    }
}

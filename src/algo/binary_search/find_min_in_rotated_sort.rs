#[allow(dead_code)]
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (left + right) / 2;

        if nums[mid] <= nums[right] {
            right = mid;
        } else {
            left = mid + 1
        }
    }

    nums[left]
}

#[cfg(test)]
mod test {
    use super::find_min;

    #[test]
    fn test_find_min1() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(find_min(nums), 1);
    }

    #[test]
    fn test_find_min2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(find_min(nums), 0);
    }

    #[test]
    fn test_find_min3() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(find_min(nums), 11);
    }

    #[test]
    fn test_find_min4() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(find_min(nums), 1);
    }
}

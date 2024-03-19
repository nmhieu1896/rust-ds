#[allow(dead_code)]
pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    let mut right = nums.len() - 1;
    let mut left = 0;

    while left < right {
        let mid = (right + left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else if nums[mid] < target {
            left = mid + 1;
        }
    }

    return if target > nums[left] { left + 1 } else { left } as i32;
}

#[cfg(test)]
mod tests {
    use super::search_insert_position;

    #[test]
    fn test_search_insert_position() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert_position(nums, target), 2);
    }
    #[test]
    fn test_search_insert_position2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert_position(nums, target), 1);
    }
    #[test]
    fn test_search_insert_position3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert_position(nums, target), 4);
    }
    #[test]
    fn test_search_insert_position4() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(search_insert_position(nums, target), 0);
    }
    #[test]
    fn test_search_insert_position5() {
        let nums = vec![1];
        let target = 1;
        assert_eq!(search_insert_position(nums, target), 0);
    }
}

#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len() - 1;
    let mut left = 0;
    let mut right = len;
    let pivot = nums[len];

    loop {
        let mid = (left + right) / 2;
        //in case right - left <=1, left == mid, so dont need to check left
        if target == nums[mid] {
            return mid as i32;
        } else if target == nums[right] {
            return right as i32;
        // exhaustive, mid and right is both not target
        } else if right - left <= 1 {
            return -1;
        }

        // mid item belong to smaller range
        // meanwhile target in bigger range
        if nums[mid] < pivot && target > pivot {
            right = mid - 1;
        // mid item belong to bigger range
        // meanwhile  target in smaller range
        } else if nums[mid] > pivot && target < pivot {
            left = mid + 1;
        //Normal case for binary search
        } else if target > nums[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
}

#[cfg(test)]
mod test_search_in_rotated_sort {
    use super::search;

    #[test]
    fn test1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn test4() {
        let nums = vec![7, 8, 0, 1, 2, 3, 4, 5];
        let target = 8;
        assert_eq!(search(nums, target), 1);
    }
    #[test]
    fn test5() {
        let nums = vec![1, 3];
        let target = 3;
        assert_eq!(search(nums, target), 1);
    }
}

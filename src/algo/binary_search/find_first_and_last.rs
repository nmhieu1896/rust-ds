#[allow(dead_code)]
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }
    if nums.len() == 1 {
        if target == nums[0] {
            return vec![0, 0];
        }
        return vec![-1, -1];
    }
    let len = nums.len() - 1;
    let mut left = 0;
    let mut right = len;
    let mut start = 0;
    let mut end = 0;

    loop {
        if right - left <= 1 {
            if nums[right] == target && nums[left] != target {
                return vec![right as i32, right as i32];
            }
            if nums[left] == target && nums[right] != target {
                return vec![left as i32, left as i32];
            }
            if nums[left] == target && nums[right] == target {
                return vec![left as i32, right as i32];
            }

            return vec![-1, -1];
        }
        let mid = (left + right) / 2;
        println!("left:{left} | right:{right} | mid:{mid}");
        if nums[mid] < target {
            left = mid + 1;
        } else if nums[mid] > target {
            right = mid - 1;
            //target == mid
        } else {
            end = mid;
            start = mid;
            break;
        }
    }
    loop {
        let mid = (left + start) / 2;
        if (start == 0 && nums[start] == target)
            || (nums[start] == target && nums[start - 1] < target)
        {
            break;
        }
        if (mid == 0 && nums[mid] == target) || (nums[mid] == target && nums[mid - 1] < target) {
            start = mid;
            break;
        }
        if nums[mid] >= target {
            start = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    loop {
        let mid = (end + right) / 2;
        if (end == len && nums[end] == target) || (nums[end] == target && nums[end + 1] > target) {
            break;
        }
        if (mid == len && nums[mid] == target) || (nums[mid] == target && nums[mid + 1] > target) {
            end = mid;
            break;
        }
        if nums[mid] <= target {
            end = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return vec![start as i32, end as i32];
}

#[cfg(test)]
mod test {
    use super::search_range;

    #[test]
    fn test1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![3, 4]);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }

    #[test]
    fn test3() {
        let nums = vec![];
        let target = 0;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }
    #[test]
    fn test4() {
        let nums = vec![1];
        let target = 0;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }
    #[test]
    fn test5() {
        let nums = vec![2, 2];
        let target = 1;
        assert_eq!(search_range(nums, target), vec![-1, -1]);
    }
    #[test]
    fn test6() {
        let nums = vec![2, 2];
        let target = 2;
        assert_eq!(search_range(nums, target), vec![0, 1]);
    }
    #[test]
    fn test7() {
        let nums = vec![1, 2];
        let target = 2;
        assert_eq!(search_range(nums, target), vec![1, 1]);
    }
}

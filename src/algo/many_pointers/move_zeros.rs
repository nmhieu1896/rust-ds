#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut ptr = 0;
    for i in 0..nums.len() {
        if nums[i] == 0 {
            continue;
        }
        (nums[ptr], nums[i]) = (nums[i], nums[ptr]);
        ptr += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::move_zeroes;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test3() {
        let mut nums = vec![0, 0, 1, 3, 0, 12, 0, 0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0, 0, 0, 0]);
    }
}

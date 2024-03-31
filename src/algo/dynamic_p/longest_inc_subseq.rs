#[allow(dead_code)]
pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = dp[i].max(dp[j] + 1);
            };
        }
    }

    return *dp.iter().max().unwrap_or(&1);
}

#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut sub_vec = vec![nums[0]];
    for i in 1..nums.len() {
        if nums[i] > *sub_vec.last().unwrap() {
            sub_vec.push(nums[i]);
            continue;
        }
        for j in 0..sub_vec.len() {
            if nums[i] <= sub_vec[j] {
                sub_vec[j] = nums[i];
                break;
            }
        }
    }

    return sub_vec.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;

    #[test]
    pub fn test1() {
        let vec = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(vec), 4);
    }

    #[test]
    pub fn test2() {
        let vec = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(length_of_lis(vec), 4);
    }

    #[test]
    pub fn test3() {
        let vec = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(length_of_lis(vec), 1);
    }

    #[test]
    pub fn test4() {
        let vec = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
        assert_eq!(length_of_lis(vec), 6);
    }
}

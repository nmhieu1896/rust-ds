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
    for n in nums.iter() {
        if n > sub_vec.last().unwrap() {
            sub_vec.push(*n);
        } else {
            // -------- use vec.binary_search
            // if let Err(idx) = sub_vec.binary_search(n) {
            //     sub_vec[idx] = *n;
            // }
            //
            // -------- use partition point  || this one currently beat 100% users in leetcode for rust.
            let idx = sub_vec.partition_point(|v| v < n);
            sub_vec[idx] = *n;
            // -------- build my own binary search
            // let idx = binary_search_left(&sub_vec, n);
            // sub_vec[idx] = *n;
        }
    }

    return sub_vec.len() as i32;
}

#[allow(dead_code)]
pub fn binary_search_left(vec: &Vec<i32>, target: &i32) -> usize {
    let mut left = 0;
    let mut right = vec.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        if vec[mid] >= *target {
            right = mid
        } else {
            left = mid + 1;
        }
    }
    return left;
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

#[allow(dead_code)]
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut nums = nums;
    nums.sort();
    let mut prev = 0;
    for &v in nums.iter() {
        if v - 1 > prev {
            for i in prev + 1..v {
                res.push(i);
            }
        }
        prev = v;
    }
    if nums[nums.len() - 1] < nums.len() as i32 {
        for i in nums[nums.len() - 1] + 1..=nums.len() as i32 {
            res.push(i);
        }
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let res = vec![5, 6];
        assert_eq!(find_disappeared_numbers(nums), res);
    }

    #[test]
    fn test_find_disappeared_numbers2() {
        let nums = vec![1, 1];
        let res = vec![2];
        assert_eq!(find_disappeared_numbers(nums), res);
    }
}

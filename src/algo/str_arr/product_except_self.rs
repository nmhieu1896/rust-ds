#[allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![1; nums.len()];
    let numlen: usize = nums.len() - 1;
    let mut prod: i32 = 1;
    // Prefix product
    for (i, num) in nums.iter().enumerate().take(numlen) {
        prod *= num;
        res[i + 1] *= prod;
    }
    prod = 1;
    // Multiply by suffix product
    for (i, num) in nums.iter().enumerate().rev().take(numlen) {
        prod *= num;
        res[i - 1] *= prod;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_product_except_self1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(product_except_self(nums), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_product_except_self2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(product_except_self(nums), vec![0, 0, 9, 0, 0]);
    }
}

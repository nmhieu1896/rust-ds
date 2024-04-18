use std::collections::HashMap;

#[allow(dead_code)]
pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashMap::new();
    let mut ans_1 = vec![];
    let mut ans_2 = vec![];
    for num in nums1 {
        map.insert(num, 1);
    }
    for num in nums2 {
        if let Some(v) = map.get(&num) {
            if v == &1 {
                map.insert(num, 3);
            }
        } else {
            map.insert(num, 2);
        }
    }

    for (k, v) in map.iter() {
        match v {
            1 => {
                ans_1.push(*k);
            }
            2 => {
                ans_2.push(*k);
            }
            _ => {}
        }
    }

    ans_1.sort();
    ans_2.sort();
    return vec![ans_1, ans_2];
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::find_difference;

    #[test]
    fn test1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let res = find_difference(nums1, nums2);
        assert_eq!(res, vec![vec![1, 3], vec![4, 6]]);
    }

    #[test]
    fn test2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let res = find_difference(nums1, nums2);
        assert_eq!(res, vec![vec![3], vec![]]);
    }

    #[test]
    fn test3() {
        let nums1 = vec![-80, -15, -81, -28, -61, 63, 14, -45, -35, -10];
        let nums2 = vec![-1, -40, -44, 41, 10, -43, 69, 10, 2];
        let res = find_difference(nums1, nums2);
        let mut ans_1 = vec![-81, -35, -10, -28, -61, -45, -15, 14, -80, 63];
        ans_1.sort();
        let mut ans_2 = vec![-1, 2, 69, -40, 41, 10, -43, -44];
        ans_2.sort();
        assert_eq!(res, vec![ans_1, ans_2]);
    }
}

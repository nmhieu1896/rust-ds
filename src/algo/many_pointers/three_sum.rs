#[allow(dead_code)]
pub fn three_sum(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if numbers.len() < 3 {
        return vec![];
    }
    let mut nums = numbers.clone();
    nums.sort();
    let mut res: Vec<Vec<i32>> = vec![];
    // let mut left = 0;
    // let mut right = numbers.len() - 1;
    // let mut mid = 0;
    // let mut checked_right = right;

    // while left >= right - 2 {
    //     if nums[left] + nums[right - 1] + nums[right] < target {
    //         left += 1;
    //         checked_right -= 1;
    //         right = checked_right;
    //         continue;
    //     }

    //     mid = left + 1;
    //     while mid < right - 1 && nums[left] + nums[mid] + nums[right] < target {
    //         mid += 1;
    //     }
    //     if nums[left] + nums[mid] + nums[right] == target {
    //         res.push(vec![nums[left], nums[mid], nums[right]]);
    //     }
    //     right = right - 1;
    // }
    for left in 0..nums.len() - 2 {
        if left > 0 && nums[left] == nums[left - 1] {
            break;
        }
        let mut right = numbers.len() - 1;
        while right >= 2 && left <= right - 2 {
            if nums.get(right) == nums.get(right + 1) {
                right -= 1;
                continue;
            }
            for mid in left + 1..right {
                if mid > left + 1 && nums.get(mid) == nums.get(mid - 1) {
                    continue;
                }
                if nums[left] + nums[mid] + nums[right] == target {
                    res.push(vec![nums[left], nums[mid], nums[right]]);
                }
            }
            right -= 1;
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn test1() {
        let numbers = vec![-1, 0, 1, 2, -1, -4];
        let target = 0;
        let res = three_sum(numbers, target);
        println!("{:?}", res);
        assert_eq!(res.len(), 2);
        assert_eq!(res.iter().all(|v| v.iter().sum::<i32>() == 0), true);
    }

    #[test]
    fn test2() {
        let numbers = vec![];
        let target = 0;
        let res = three_sum(numbers, target);

        assert_eq!(res, vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn test3() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let target = 10;
        let res = three_sum(numbers, target);
        println!("{:?}", res);
        assert_eq!(res.len(), 4);
        assert_eq!(res.iter().all(|v| v.iter().sum::<i32>() == 10), true);
        // assert_eq!(
        //     res,
        //     vec![vec![1, 2, 7], vec![1, 3, 6], vec![1, 4, 5], vec![2, 3, 5]]
        // );
    }
    #[test]
    fn test4() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let target = 12;
        let res = three_sum(numbers, target);
        println!("{:?}", res);
        assert_eq!(res.len(), 6);
        assert_eq!(res.iter().all(|v| v.iter().sum::<i32>() == target), true);
        // assert_eq!(
        //     res,
        //     vec![vec![1, 4, 7], vec![1, 5, 6], vec![2, 3, 7], vec![2, 4, 6], vec![3, 4, 5]]
        // );
    }

    #[test]
    fn test5() {
        let numbers = vec![0, 0, 0, 0];
        let target = 0;
        let res = three_sum(numbers, target);

        assert_eq!(res, vec![vec![0, 0, 0]]);
        // assert_eq!(
        //     res,
        //     vec![vec![1, 4, 7], vec![1, 5, 6], vec![2, 3, 7], vec![2, 4, 6], vec![3, 4, 5]]
        // );
    }
}

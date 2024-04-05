#[allow(dead_code)]
pub fn three_sum(numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut numbers = numbers;
    numbers.sort();
    let mut res = vec![];
    for i in 0..numbers.len() {
        if i > 0 && numbers[i] == numbers[i - 1] {
            continue;
        }
        let mut left = i + 1;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = numbers[i] + numbers[left] + numbers[right];
            if sum == target {
                res.push(vec![numbers[i], numbers[left], numbers[right]]);
                while left < right && numbers[left] == numbers[left + 1] {
                    left += 1;
                }
                while left < right && numbers[right] == numbers[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    res
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

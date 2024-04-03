#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ptr = numbers.len() - 1;
    for (i, v) in numbers.iter().enumerate() {
        while ptr > 0 && v + numbers[ptr] > target {
            ptr -= 1;
        }
        if v + numbers[ptr] == target {
            return vec![i as i32 + 1, ptr as i32 + 1];
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let res = two_sum(numbers, target);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let numbers = vec![2, 3, 4, 7, 10];
        let target = 9;
        let res = two_sum(numbers, target);
        assert_eq!(res, vec![1, 4]);
    }

    #[test]
    fn test3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let res = two_sum(numbers, target);
        assert_eq!(res, vec![1, 2]);
    }
}

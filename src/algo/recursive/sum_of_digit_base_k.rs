#[allow(dead_code)]
pub fn sum_base(n: i32, k: i32) -> i32 {
    let new_base = to_base(n, k);
    return sum_digits(new_base);
}

pub fn to_base(n: i32, k: i32) -> i32 {
    if n < k {
        return n;
    } else {
        return to_base(n / k, k) * 10 + (n % k);
    }
}

pub fn sum_digits(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else {
        return n % 10 + sum_digits(n / 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base_sum_digits() {
        assert_eq!(to_base(34, 6), 54);
        assert_eq!(to_base(45, 6), 113);
        assert_eq!(sum_digits(54), 9);
        assert_eq!(sum_digits(544), 13);
    }

    #[test]
    fn test_sum_base() {
        let n = 34;
        let k = 6;
        let res = sum_base(n, k);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_sum_base2() {
        let n = 10;
        let k = 10;
        let res = sum_base(n, k);
        assert_eq!(res, 1);
    }
}

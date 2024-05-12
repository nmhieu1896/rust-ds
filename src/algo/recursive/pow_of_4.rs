pub fn is_power_of_four(n: i32) -> bool {
    if n == 0 {
        return false;
    } else if n == 1 {
        return true;
    } else if n % 4 != 0 {
        return false;
    }

    return is_power_of_four(n / 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four() {
        let n = 16;
        let res = is_power_of_four(n);
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_power_of_four2() {
        let n = 5;
        let res = is_power_of_four(n);
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_power_of_four3() {
        let n = 1;
        let res = is_power_of_four(n);
        assert_eq!(res, true);
    }
}

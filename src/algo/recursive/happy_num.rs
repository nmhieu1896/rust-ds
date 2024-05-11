use std::collections::HashSet;

#[allow(dead_code)]
pub fn is_happy(n: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    rec_happy(n, &mut set)
}
pub fn rec_happy(n: i32, set: &mut HashSet<i32>) -> bool {
    if set.contains(&n) {
        return false;
    }
    let mut sum = 0;
    let mut mut_n = n;
    while mut_n > 0 {
        let digit = mut_n % 10;
        sum += digit * digit;
        mut_n /= 10;
    }
    if sum == 1 {
        true
    } else {
        set.insert(n);
        rec_happy(sum, set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        let n = 19;
        let res = is_happy(n);
        assert_eq!(res, true);
    }

    #[test]
    fn test_is_happy2() {
        let n = 2;
        let res = is_happy(n);
        assert_eq!(res, false);
    }
    #[test]
    fn test_is_happy3() {
        let n = 7;
        let res = is_happy(n);
        assert_eq!(res, true);
    }
}

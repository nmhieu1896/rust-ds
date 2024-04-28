#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    return s;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_palindrome1() {
        let s = "babad";
        assert_eq!(longest_palindrome(s.to_string()), "bab");
    }

    #[test]
    fn test_longest_palindrome2() {
        let s = "cbbd";
        assert_eq!(longest_palindrome(s.to_string()), "bb");
    }
}

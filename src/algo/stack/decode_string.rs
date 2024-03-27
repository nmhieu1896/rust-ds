#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    // let mut stack = Vec::new();
    // let mut num = 0;
    // let mut str = String::new();

    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "3[a]2[bc]".to_string();
        assert_eq!(decode_string(s), "aaabcbc".to_string());
    }

    #[test]
    fn test_2() {
        let s = "3[a2[c]]".to_string();
        assert_eq!(decode_string(s), "accaccacc".to_string());
    }

    #[test]
    fn test_3() {
        let s = "2[abc]3[cd]ef".to_string();
        assert_eq!(decode_string(s), "abcabccdcdcdef".to_string());
    }

    #[test]
    fn test_4() {
        let s = "abc3[cd]xyz".to_string();
        assert_eq!(decode_string(s), "abccdcdcdxyz".to_string());
    }

    #[test]
    fn test_5() {
        let s = "100[leetcode]".to_string();
        assert_eq!(decode_string(s), "leetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcodeleetcode".to_string());
    }
}

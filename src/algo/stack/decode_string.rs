#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    let mut stack = Vec::new();
    let mut num = 0;
    let mut str = String::new();

    s.chars().into_iter().for_each(|c| match c {
        x if x.is_digit(10) => {
            // num * 10 for handling number that > 10 (test_5)
            num = num * 10 + x.to_digit(10).unwrap();
        }
        x if x == '[' => {
            stack.push((str.clone(), num));
            num = 0;
            str.clear();
        }
        x if x == ']' => {
            let (pre_str, count) = stack.pop().unwrap();
            str = pre_str + &str.repeat(count as usize)
        }
        x => {
            str.push(x);
        }
    });

    str
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

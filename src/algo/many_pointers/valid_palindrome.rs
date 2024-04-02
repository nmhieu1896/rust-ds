#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    let vec = s.chars().collect::<Vec<char>>();

    while i < j {
        let char_i = vec[i];
        let char_j = vec[j];

        if !char_i.is_alphanumeric() {
            i += 1;
            continue;
        }
        if !char_j.is_alphanumeric() {
            j -= 1;
            continue;
        }

        if char_i.to_ascii_lowercase() != char_j.to_ascii_lowercase() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test1() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test3() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn test4() {
        assert_eq!(is_palindrome("0P".to_string()), false);
    }

    #[test]
    fn test5() {
        assert_eq!(is_palindrome("a".to_string()), true);
    }
}

#[allow(dead_code)]
pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = vec![];
    let mut new_str = String::new();

    s.chars().into_iter().for_each(|char| match char {
        c if c == '(' => {
            stack.push(new_str.clone());
            new_str.clear();
        }
        c if c == ')' => {
            if stack.last().is_some() {
                new_str = stack.pop().unwrap() + "(" + &new_str + ")";
            }
        }
        c => new_str.push(c),
    });

    while stack.len() > 0 {
        new_str = stack.pop().unwrap() + &new_str;
    }

    new_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "lee(t(c)o)de)".to_string();
        assert_eq!(min_remove_to_make_valid(s), "lee(t(c)o)de".to_string());
    }

    #[test]
    fn test_2() {
        let s = "a)b(c)d".to_string();
        assert_eq!(min_remove_to_make_valid(s), "ab(c)d".to_string());
    }

    #[test]
    fn test_3() {
        let s = "))((".to_string();
        assert_eq!(min_remove_to_make_valid(s), "".to_string());
    }

    #[test]
    fn test_4() {
        let s = "(a(b(c)d)".to_string();
        assert_eq!(min_remove_to_make_valid(s), "a(b(c)d)".to_string());
    }

    #[test]
    fn test_5() {
        let s = "gg(a(b(c)d)".to_string();
        assert_eq!(min_remove_to_make_valid(s), "gga(b(c)d)".to_string());
    }
}

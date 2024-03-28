#[allow(dead_code)]
pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack = vec![];
    let mut new_str = String::new();
    let mut remove_idx = vec![false; s.len()];

    s.chars().enumerate().for_each(|(i, char)| match char {
        c if c == '(' => stack.push(i),
        c if c == ')' => {
            if stack.last().is_some() {
                stack.pop();
            } else {
                remove_idx[i] = true;
            }
        }
        _ => {}
    });

    while stack.len() > 0 {
        remove_idx[stack.pop().unwrap()] = true
    }

    s.chars().enumerate().for_each(|(i, char)| {
        if !remove_idx[i] {
            new_str.push(char);
        }
    });

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

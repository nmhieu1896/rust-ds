#[allow(dead_code)]
pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let mut max = 0;
    let mut vec = vec![];
    let mut break_point = (' ', 0);
    let mut from = 0;

    s.chars().enumerate().for_each(|(idx, c)| {
        if vec.len() == 0 || (vec.len() == 1 && c != vec[0]) {
            vec.push(c);
            max = idx + 1;
        } else if vec.contains(&c) {
            if (idx + 1 - from) > max {
                max = idx + 1 - from;
            }
        } else {
            if break_point.0 == vec[0] {
                vec[1] = c
            } else {
                vec[0] = c
            }

            from = break_point.1;
        }

        if c != break_point.0 {
            break_point = (c, idx);
        }
    });

    return max as i32;
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring_two_distinct;

    #[test]
    fn test1() {
        let s = "eceba".to_string();
        let res = length_of_longest_substring_two_distinct(s);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let s = "ccaabbb".to_string();
        let res = length_of_longest_substring_two_distinct(s);
        assert_eq!(res, 5);
    }

    #[test]
    fn test3() {
        let s = "aac".to_string();
        let res = length_of_longest_substring_two_distinct(s);
        assert_eq!(res, 3);
    }

    #[test]
    fn test4() {
        let s = "abbbaaaaccccc".to_string();
        let res = length_of_longest_substring_two_distinct(s);
        assert_eq!(res, 9);
    }
}

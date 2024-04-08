use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring_two_distinct(s: String, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }
    let mut max = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut right = 0;

    s.chars().enumerate().for_each(|(idx, c)| {
        if map.contains_key(&c) || map.keys().len() < (k as usize) {
            *map.entry(c).or_insert(0) = idx;
            right = idx;
            max = max.max(right - left + 1);
        } else {
            left = *map.values().min().unwrap() + 1;
            map.retain(|_, v| *v != left - 1);
            map.insert(c, idx);
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
        let res = length_of_longest_substring_two_distinct(s, 2);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let s = "ccaabbb".to_string();
        let res = length_of_longest_substring_two_distinct(s, 2);
        assert_eq!(res, 5);
    }

    #[test]
    fn test3() {
        let s = "aac".to_string();
        let res = length_of_longest_substring_two_distinct(s, 2);
        assert_eq!(res, 3);
    }

    #[test]
    fn test4() {
        let s = "abbbaaaaccccc".to_string();
        let res = length_of_longest_substring_two_distinct(s, 2);
        assert_eq!(res, 9);
    }
    #[test]
    fn test5() {
        let s = "ababacccccc".to_string();
        let res = length_of_longest_substring_two_distinct(s, 2);
        assert_eq!(res, 7);
    }
    #[test]
    fn test6() {
        let s = "a".to_string();
        let res = length_of_longest_substring_two_distinct(s, 0);
        assert_eq!(res, 0);
    }
}

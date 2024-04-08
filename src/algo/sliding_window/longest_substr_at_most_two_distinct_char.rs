use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let mut max = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut right = 0;

    s.chars().enumerate().for_each(|(idx, c)| {
        if map.contains_key(&c) || map.keys().len() < 2 {
            map.remove(&c);
            map.insert(c, idx);
            right = idx;
            max = max.max(right - left + 1);
        } else {
            let mut min_v = idx;
            let mut min_k = ' ';
            for (&k, &v) in &map {
                if v < min_v {
                    min_v = v;
                    min_k = k;
                }
            }

            left = *map.values().min().unwrap() + 1;
            map.remove(&min_k);
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
    #[test]
    fn test5() {
        let s = "ababacccccc".to_string();
        let res = length_of_longest_substring_two_distinct(s);
        assert_eq!(res, 7);
    }
}

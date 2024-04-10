use std::collections::HashMap;

#[allow(dead_code)]
pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
    if s.len() < k as usize {
        return 0;
    }
    let mut map = HashMap::new();
    let mut count = 0;
    let s_bytes = s.as_bytes();
    let mut r = 0;

    for (l, v) in s_bytes.iter().enumerate() {
        while r - l < k as usize {
            map.entry(s_bytes[r]).and_modify(|v| *v += 1).or_insert(1);
            if map.len() == k as usize {
                count += 1;
            }

            if r == s.len() - 1 {
                return count;
            }
            r += 1;
        }
        match map.get(&v) {
            Some(&value) if value > 1 => {
                map.entry(*v).and_modify(|v| *v -= 1);
            }
            _ => {
                map.remove(&v);
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::num_k_len_substr_no_repeats;

    #[test]
    fn test1() {
        let s = "havefunonleetcode".to_string();
        let k = 5;
        let res = num_k_len_substr_no_repeats(s, k);
        assert_eq!(res, 6);
    }

    #[test]
    fn test2() {
        let s = "home".to_string();
        let k = 5;
        let res = num_k_len_substr_no_repeats(s, k);
        assert_eq!(res, 0);
    }

    #[test]
    fn test3() {
        let s = "havefunonleetcode".to_string();
        let k = 3;
        let res = num_k_len_substr_no_repeats(s, k);
        assert_eq!(res, 12);
    }

    #[test]
    fn test4() {
        let s = "havefunonleetcode".to_string();
        let k = 2;
        let res = num_k_len_substr_no_repeats(s, k);
        assert_eq!(res, 15);
    }

    #[test]
    fn test5() {
        let s = "havefunonleetcode".to_string();
        let k = 1;
        let res = num_k_len_substr_no_repeats(s, k);
        assert_eq!(res, 17);
    }
}

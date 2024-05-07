#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let vec_b: Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();

    let mut res = String::new();
    for i in 0.. {
        for s in vec_b.iter() {
            if s.len() <= i {
                return res;
            } else if s[i] != vec_b[0][i] {
                return res;
            }
        }
        res.push(char::from_u32(vec_b[0][i] as u32).unwrap());
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix1() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let res = "fl".to_string();
        assert_eq!(longest_common_prefix(strs), res);
    }

    #[test]
    fn test_longest_common_prefix2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}

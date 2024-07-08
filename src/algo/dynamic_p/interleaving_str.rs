use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }

        let mut cache: HashMap<(usize, usize), bool> = HashMap::new();
        return Self::dp(s1.as_bytes(), s2.as_bytes(), s3.as_bytes(), &mut cache);
    }

    pub fn dp(s1: &[u8], s2: &[u8], s3: &[u8], cache: &mut HashMap<(usize, usize), bool>) -> bool {
        if s3.len() == 0 {
            return true;
        }
        if let Some(val) = cache.get(&(s1.len(), s2.len())) {
            return *val;
        }

        let mut res = false;
        if s1.get(0) == s3.get(0) && s2.get(0) == s3.get(0) {
            res = Self::dp(&s1[1..], &s2, &s3[1..], cache)
                || Self::dp(&s1, &s2[1..], &s3[1..], cache);
        } else if s1.get(0) == s3.get(0) {
            res = Self::dp(&s1[1..], &s2, &s3[1..], cache);
        } else if s2.get(0) == s3.get(0) {
            res = Self::dp(&s1, &s2[1..], &s3[1..], cache);
        }
        cache.insert((s1.len(), s2.len()), res);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_interleave(
                "abef".to_string(),
                "abcd".to_string(),
                "abcdbef".to_string()
            ),
            false
        );
    }

    #[test]
    fn test3() {
        let x = Instant::now();
        Solution::is_interleave(
                "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
                "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string(),
                "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string()
            );
        let time = x.elapsed();
        println!("{time:?}");
        assert!(time < Duration::from_millis(30));
    }
}

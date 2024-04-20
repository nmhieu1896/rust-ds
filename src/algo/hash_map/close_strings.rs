#[allow(dead_code)]
pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut map1 = [0; 26];
    let mut map2 = [0; 26];
    for c in word1.bytes() {
        map1[(c - b'a') as usize] += 1;
    }
    for c in word2.bytes() {
        map2[(c - b'a') as usize] += 1;
    }
    for i in 0..26 {
        if (map1[i] == 0 && map2[i] != 0) || (map1[i] != 0 && map2[i] == 0) {
            return false;
        }
    }

    map1.sort();
    map2.sort();
    return map1 == map2;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_close_strings() {
        assert_eq!(close_strings("abc".to_string(), "bca".to_string()), true);
        assert_eq!(close_strings("a".to_string(), "aa".to_string()), false);
        assert_eq!(
            close_strings("cabbba".to_string(), "abbccc".to_string()),
            true
        );
        assert_eq!(
            close_strings("cabbba".to_string(), "aabbss".to_string()),
            false
        );
    }
    #[test]
    fn test_close_strings2() {
        assert_eq!(
            close_strings("abbzzca".to_string(), "babzzcz".to_string()),
            false
        );
    }
}

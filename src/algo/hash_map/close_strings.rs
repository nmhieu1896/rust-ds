use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    // let mut set1 = HashSet::new();
    // let mut set2 = HashSet::new();
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for c in word1.bytes() {
        *map1.entry(c).or_insert(0) += 1;
        // set1.insert(c);
    }
    for c in word2.bytes() {
        *map2.entry(c).or_insert(0) += 1;
        // set1.insert(c);
    }
    for (k, v1) in map1 {
        vec1.push(v1);
        if !map2.contains_key(&k) {
            return false;
        }
        vec2.push(map2[&k]);
    }
    vec1.sort();
    vec2.sort();
    return vec1 == vec2;
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

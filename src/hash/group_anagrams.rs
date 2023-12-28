use std::{
    collections::{hash_map, HashMap},
    vec,
};

pub fn _run() {
    let exp1 = to_vec_string(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
    let exp2 = to_vec_string(vec![""]);
    let exp3 = to_vec_string(vec!["a"]);
    let test_cases = vec![exp1, exp2, exp3];
    for test_case in test_cases.iter() {
        let result = group_anagrams(test_case.clone());
        println!("result {:?}", result);
    }
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = hash_map::HashMap::new();
    for text in strs.iter() {
        let mut chars = text.clone().chars().collect::<Vec<char>>();
        chars.sort();
        map.entry(chars)
            .and_modify(|v| {
                v.push(text.to_string());
            })
            .or_insert(vec![text.to_string()]);
    }
    let result: Vec<Vec<String>> = map.values().into_iter().map(|v| v.clone()).collect();

    return result;
}

fn to_vec_string(vec: Vec<&str>) -> Vec<String> {
    return vec.iter().map(|s| s.to_string()).collect();
}

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
        let result = _group_anagrams(test_case.clone());
        println!("result {:?}", result);
    }
    for test_case in test_cases.iter() {
        let result = _group_anagrams_2(test_case.clone());
        println!("result {:?}", result);
    }
}

fn _group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = hash_map::HashMap::new();
    for text in strs.iter() {
        let mut chars = text.clone().chars().collect::<Vec<char>>();
        chars.sort();
        map.entry(chars).or_insert(vec![]).push(text.to_string());
    }
    let result: Vec<Vec<String>> = map.values().cloned().collect();

    return result;
}

fn to_vec_string(vec: Vec<&str>) -> Vec<String> {
    return vec.iter().map(|s| s.to_string()).collect();
}

const N_LETTERS: usize = (b'z' - b'a' + 1) as _;

pub fn _group_anagrams_2(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.into_iter()
        .fold(
            HashMap::<[u8; N_LETTERS], Vec<String>>::new(),
            |mut map, s| {
                let freqs = s.bytes().map(|b| (b - b'a') as usize).fold(
                    [0; N_LETTERS],
                    |mut freqs, bin| {
                        freqs[bin] += 1;
                        freqs
                    },
                );
                map.entry(freqs).or_default().push(s);
                map
            },
        )
        .into_values()
        .collect()
}

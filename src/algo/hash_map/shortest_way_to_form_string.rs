#[allow(dead_code)]
pub fn shortest_way(source: String, target: String) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut result = 0;
    for (i, c) in source.bytes().enumerate() {
        map.entry(c).or_insert(vec![]).push(i);
    }
    let mut idx = -1;
    for (i, c) in target.bytes().enumerate() {
        if !map.contains_key(&c) {
            return -1;
        }
        let val = map.get(&c).unwrap();
        let next_idx = val.iter().find(|&&x| (x as i32) > idx);

        if next_idx.is_none() {
            result += 1;
            idx = *val.get(0).unwrap() as i32;
        } else {
            idx = *next_idx.unwrap() as i32;
        }

        if i == target.len() - 1 {
            result += 1;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_way1() {
        let source = "abc";
        let target = "abcbc";
        assert_eq!(shortest_way(source.to_string(), target.to_string()), 2);
    }

    #[test]
    fn test_shortest_way2() {
        let source = "abc";
        let target = "acdbc";
        assert_eq!(shortest_way(source.to_string(), target.to_string()), -1);
    }

    #[test]
    fn test_shortest_way3() {
        let source = "xyz";
        let target = "xzyxz";
        assert_eq!(shortest_way(source.to_string(), target.to_string()), 3);
    }

    #[test]
    fn test_shortest_way4() {
        let source = "bxdisnclwdrpcqamhqqvudgtdbsdikhzzbnlgzlspozvhdkunxkpevnqvyrfowanagolpwvezuvnhgxvjopcvrkdaippmwgkofbo";
        let target =
            "ntzebqmlrzxissncdlvcxbojgbnnphtfdunjpzroegfdvieaajafjkidpxbrgsjpgmalekhjckwgygfz";
        assert_eq!(shortest_way(source.to_string(), target.to_string()), 17);
    }
}

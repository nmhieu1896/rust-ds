#[allow(dead_code)]
pub fn shortest_way(source: String, target: String) -> i32 {
    return 0;
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
}

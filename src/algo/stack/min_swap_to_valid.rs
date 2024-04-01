#[allow(dead_code)]
pub fn min_swaps(s: String) -> i32 {
    let mut num_open = 0;
    let mut num_close = 0;
    for c in s.chars() {
        if c == '[' {
            num_open += 1;
            continue;
        }

        if num_open == 0 {
            num_close += 1;
        } else {
            num_open -= 1;
        }
    }

    return num_close / 2 + num_close % 2;
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    pub fn test1() {
        let s = "][][".to_string();
        assert_eq!(min_swaps(s), 1);
    }

    #[test]
    pub fn test2() {
        let s = "]]][[[".to_string();
        // - Swap index 0 with index 4. s = "[]][][".
        // - Swap index 1 with index 5. s = "[[][]]".
        assert_eq!(min_swaps(s), 2);
    }
    #[test]
    pub fn test3() {
        let s = "]]]][[[[".to_string();
        // - Swap index 0 with index 4. s = "[]]] [[][".
        // - Swap index 1 with index 5. s = "[[][]]".
        assert_eq!(min_swaps(s), 2);
    }

    #[test]
    pub fn test4() {
        let s = "[]".to_string();
        assert_eq!(min_swaps(s), 0);
    }

    #[test]
    pub fn test5() {
        let s = "][[]".to_string();
        assert_eq!(min_swaps(s), 1);
    }

    #[test]
    pub fn test6() {
        let s = "][[[]]][".to_string();
        assert_eq!(min_swaps(s), 1);
    }
}

#[allow(dead_code)]
pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack = vec![];
    let mut rain = 0;

    for (i, h) in height.iter().enumerate() {
        while !stack.is_empty() && *h > height[*stack.last().unwrap()] {
            let prev_h = stack.pop().unwrap();
            if stack.is_empty() {
                break;
            }
            let distance = i - stack.last().unwrap() - 1;
            let h_diff = h.min(&height[*stack.last().unwrap()]) - height[prev_h];
            rain += (distance as i32) * h_diff;
        }
        stack.push(i);
    }

    return rain;
}

#[cfg(test)]
mod tests {
    use super::trap;

    #[test]
    pub fn test1() {
        let vec = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(vec), 6);
    }

    #[test]
    pub fn test2() {
        let vec = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(vec), 9);
    }

    #[test]
    pub fn test3() {
        let vec = vec![4, 2, 3];
        assert_eq!(trap(vec), 1);
    }

    #[test]
    pub fn test4() {
        let vec = vec![4, 2, 3, 1];
        assert_eq!(trap(vec), 1);
    }

    #[test]
    pub fn test5() {
        let vec = vec![4, 2, 3, 1, 3];
        assert_eq!(trap(vec), 3);
    }

    #[test]
    pub fn test6() {
        let vec = vec![4, 2, 3, 1, 3, 2];
        assert_eq!(trap(vec), 3);
    }
}

#[allow(dead_code)]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut result = vec![0; temperatures.len()];

    for i in 0..temperatures.len() {
        while stack.len() > 0 && temperatures[i] > temperatures[*stack.last().unwrap()] {
            let idx = stack.pop().unwrap();
            result[idx] = (i - idx) as i32;
        }
        stack.push(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_2() {
        let temperatures = vec![30, 40, 50, 60];
        let result = vec![1, 1, 1, 0];
        assert_eq!(daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_3() {
        let temperatures = vec![30, 60, 90];
        let result = vec![1, 1, 0];
        assert_eq!(daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_4() {
        let temperatures = vec![30, 30, 30, 30];
        let result = vec![0, 0, 0, 0];
        assert_eq!(daily_temperatures(temperatures), result);
    }

    #[test]
    fn test_5() {
        let temperatures = vec![30, 30, 30, 60];
        let result = vec![3, 2, 1, 0];
        assert_eq!(daily_temperatures(temperatures), result);
    }
}

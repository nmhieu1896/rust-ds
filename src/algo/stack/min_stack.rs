#[allow(dead_code)]
struct MinStack {
    value: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        MinStack {
            value: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.value.push(val);
        self.min.push(*self.min.last().unwrap_or(&val).min(&val));
    }

    fn pop(&mut self) {
        self.value.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        *self.value.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }
}

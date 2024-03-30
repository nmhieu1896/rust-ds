#[derive(PartialEq, Clone, Debug)]
enum Mod {
    Add,
    Neg,
}
#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    let mut stack: Vec<(Vec<i32>, Mod)> = vec![(vec![], Mod::Add)];
    let mut num: i32 = 0;
    let mut curr_mod = Mod::Add;
    let mut nested_stack = vec![];

    s.chars().into_iter().for_each(|char| {
        if char.is_digit(10) {
            num = num * 10 + char.to_digit(10).unwrap() as i32;
            return;
        }
        if num != 0 {
            let mul_val = if curr_mod == Mod::Add { 1 } else { -1 };
            stack.last_mut().unwrap().0.push(num * mul_val);
            num = 0;
        }

        match char {
            c if c == '+' => {
                curr_mod = Mod::Add;
            }
            c if c == '-' => {
                curr_mod = Mod::Neg;
            }
            c if c == '(' => {
                // Handle nested mode,
                // if outer () is Neg, reverse mod for nested ()
                let nested_mod = match nested_stack.last() {
                    Some(Mod::Add) => curr_mod.clone(),
                    Some(Mod::Neg) => match curr_mod {
                        Mod::Add => Mod::Neg,
                        Mod::Neg => Mod::Add,
                    },
                    None => curr_mod.clone(),
                };
                stack.push((vec![], nested_mod.clone()));
                nested_stack.push(nested_mod);
                curr_mod = Mod::Add;
            }
            c if c == ')' => {
                nested_stack.pop();
                // If nested_stack still have item => still in nested ()
                // Mod must be the mod of outer ().
                curr_mod = if nested_stack.last().is_some() {
                    nested_stack.last().unwrap().clone()
                } else {
                    Mod::Add
                };
                stack.push((vec![], curr_mod.clone()));
            }
            _ => {}
        }
    });
    if num != 0 {
        let mul_val = if curr_mod == Mod::Add { 1 } else { -1 };
        stack.last_mut().unwrap().0.push(num * mul_val);
    }

    let mut sum: i32 = 0;
    stack
        .iter()
        .map(|item| (item.0.iter().sum::<i32>(), item.1.clone()))
        .for_each(|(value, curr_mod)| match curr_mod {
            Mod::Add => {
                sum += value;
            }
            Mod::Neg => {
                sum -= value;
            }
        });
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "(10+(4+(5+2))-3) + (6+8)".to_string();
        assert_eq!(calculate(s), 32);
    }

    #[test]
    fn test_2() {
        let s = "(1+ (4+5 - 10) - (3+2))".to_string();
        assert_eq!(calculate(s), -5);
    }

    #[test]
    fn test_3() {
        let s = "(1 + 1)".to_string();
        assert_eq!(calculate(s), 2);
    }

    #[test]
    fn test_4() {
        let s = "-(1 + 1) + 1".to_string();
        assert_eq!(calculate(s), -1);
    }
    #[test]
    fn test_5() {
        let s = "- (3 + (4 + 5))".to_string();
        assert_eq!(calculate(s), -12);
    }

    #[test]
    fn test_6() {
        let s = "2-4-(8+2-6+(8+4-(1)+8-10))".to_string();
        assert_eq!(calculate(s), -15);
    }
}

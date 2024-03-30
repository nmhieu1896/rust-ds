#[derive(PartialEq, Clone, Debug)]
enum Mod {
    Add,
    Neg,
}
#[derive(Debug)]
enum StackItem {
    Value(i32),
    Operand((), Mod),
}

#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    let mut stack: Vec<StackItem> = vec![];
    let mut num: String = String::new();
    let mut curr_mod = Mod::Add;

    s.chars().into_iter().for_each(|char| {
        if char.is_digit(10) {
            if curr_mod == Mod::Neg {
                num.push('-');
                curr_mod = Mod::Add;
            }
            num.push(char);
            return;
        }
        if !num.is_empty() {
            stack.push(StackItem::Value(num.parse::<i32>().unwrap()));
            num.clear();
        }

        match char {
            c if c == '+' => {
                curr_mod = Mod::Add;
            }
            c if c == '-' => {
                curr_mod = Mod::Neg;
            }
            c if c == '(' => {
                stack.push(StackItem::Operand((), curr_mod.clone()));
                curr_mod = Mod::Add;
            }
            c if c == ')' => loop {
                curr_mod = Mod::Add;
                let item = stack.pop().unwrap();
                match item {
                    StackItem::Value(val) => match stack.last().unwrap() {
                        StackItem::Operand(_, outer_mod) => {
                            if *outer_mod == Mod::Neg {
                                stack.pop().unwrap();
                                stack.push(StackItem::Value(val * (-1)));
                            } else {
                                stack.pop().unwrap();
                                stack.push(StackItem::Value(val));
                            }
                            break;
                        }
                        StackItem::Value(pre_val) => {
                            *stack.last_mut().unwrap() = StackItem::Value(val + pre_val)
                        }
                    },
                    _ => {}
                };
            },
            _ => {}
        }
    });
    if !num.is_empty() {
        let mul_val = if curr_mod == Mod::Add { 1 } else { -1 };
        stack.push(StackItem::Value(num.parse::<i32>().unwrap() * mul_val));
    }

    stack
        .iter()
        .map(|item| match item {
            StackItem::Value(val) => *val,
            _ => 0,
        })
        .sum()
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

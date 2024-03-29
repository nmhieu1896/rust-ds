enum Mod {
    Add,
    Neg,
    Mul,
    Div,
}
#[allow(dead_code)]
pub fn calculate(s: String) -> i32 {
    let mut stack: Vec<i32> = vec![];
    let mut num: i32 = 0;
    let mut curr_mod = Mod::Add;

    s.chars().into_iter().for_each(|char| match char {
        c if c.is_digit(10) => {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
        }
        c if c == '+' => {
            handle_stack(&mut stack, &mut num, &mut curr_mod);
            curr_mod = Mod::Add;
        }
        c if c == '-' => {
            handle_stack(&mut stack, &mut num, &mut curr_mod);
            curr_mod = Mod::Neg;
        }
        c if c == '*' => {
            handle_stack(&mut stack, &mut num, &mut curr_mod);
            curr_mod = Mod::Mul;
        }
        c if c == '/' => {
            handle_stack(&mut stack, &mut num, &mut curr_mod);
            curr_mod = Mod::Div;
        }
        _ => {}
    });
    handle_stack(&mut stack, &mut num, &mut curr_mod);

    stack.iter().sum()
}

fn handle_stack(vec: &mut Vec<i32>, num: &mut i32, curr_mod: &mut Mod) {
    match curr_mod {
        Mod::Add => {
            vec.push(*num);
        }
        Mod::Neg => vec.push(*num * (-1)),
        Mod::Mul => {
            let last = vec.last_mut().unwrap();
            *last *= *num;
        }
        Mod::Div => {
            let last = vec.last_mut().unwrap();
            *last /= *num;
        }
    };
    *num = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "3+2*2".to_string();
        assert_eq!(calculate(s), 7);
    }

    #[test]
    fn test_2() {
        let s = " 3/2 ".to_string();
        assert_eq!(calculate(s), 1);
    }

    #[test]
    fn test_3() {
        let s = " 3+5 / 2 ".to_string();
        assert_eq!(calculate(s), 5);
    }

    #[test]
    fn test_4() {
        let s = "23 + 25 / 2 ".to_string();
        assert_eq!(calculate(s), 35);
    }

    #[test]
    fn test_5() {
        let s = "23 + 25 * 12 / 2 ".to_string();
        assert_eq!(calculate(s), 173);
    }
}

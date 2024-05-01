use std::collections::HashMap;

#[allow(dead_code)]
fn parser(str: String) -> Json {
    let mut iter_str = str.chars();
    loop {
        let c = iter_str.next().unwrap();
        let mut new_str = String::new();

        if c.is_digit(10) {
            new_str.push(c);
            loop {
                let next_c = iter_str.next();
                if (next_c.is_some() && next_c.unwrap().is_digit(10)) || next_c == Some('.') {
                    new_str.push(next_c.unwrap());
                } else {
                    break;
                }
            }
            return Json::Number(new_str.parse::<f64>().unwrap());
        }
        if c == '"' {
            loop {
                let next_c = iter_str.next();
                if next_c.is_none() || next_c == Some('"') {
                    break;
                } else {
                    new_str.push(next_c.unwrap());
                }
            }
            return Json::String(new_str.to_string());
        }
        if c == 't' {
            new_str.push(c);
            for _ in 0..3 {
                new_str.push(iter_str.next().unwrap());
            }
            if new_str == "true" {
                return Json::Boolean(true);
            }
        }
        if c == 'f' {
            new_str.push(c);
            for _ in 0..4 {
                new_str.push(iter_str.next().unwrap());
            }
            if new_str == "false" {
                return Json::Boolean(false);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Json {
    Object(HashMap<String, Box<Json>>),
    Array(Vec<Box<Json>>),
    Number(f64),
    String(String),
    Boolean(bool),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser1() {
        let str = r#""Hello world""#;
        assert_eq!(
            parser(str.to_string()),
            Json::String("Hello world".to_string())
        );
    }
    #[test]
    fn test_parser2() {
        let str = "1024";
        assert_eq!(parser(str.to_string()), Json::Number(1024 as f64));
    }

    #[test]
    fn test_parser3() {
        let str1 = "true";
        let str2 = "false";
        assert_eq!(parser(str1.to_string()), Json::Boolean(true));
        assert_eq!(parser(str2.to_string()), Json::Boolean(false));
    }
}

use std::collections::HashMap;

#[allow(dead_code)]
fn parser(str: &str) -> (Json, Option<&str>) {
    let mut iter_str = str.chars().enumerate();
    loop {
        let (i, c) = iter_str.next().unwrap();
        let mut new_str = String::new();
        let mut break_point: Option<usize> = None;

        if c.is_digit(10) {
            new_str.push(c);
            loop {
                let next_c = iter_str.next();
                if next_c.is_some() {
                    let (i, next_c) = next_c.unwrap();
                    if next_c.is_digit(10) || next_c == '.' {
                        new_str.push(next_c);
                    } else {
                        break_point = Some(i);
                        break;
                    }
                } else {
                    break_point = None;
                    break;
                }
            }
            return (
                Json::Number(new_str.parse::<f64>().unwrap()),
                get_slice_from_breakpoint(str, break_point),
            );
        } else if c == '"' {
            loop {
                let next_c = iter_str.next();
                if next_c.is_none() {
                    break_point = None;
                    break;
                } else {
                    let (i, next_c) = next_c.unwrap();
                    if next_c != '"' {
                        new_str.push(next_c);
                    } else {
                        break_point = Some(i + 1);
                        break;
                    }
                }
            }
            return (
                Json::String(new_str.to_string()),
                get_slice_from_breakpoint(str, break_point),
            );
        } else if c == 't' {
            new_str.push(c);
            for _ in 0..3 {
                new_str.push(iter_str.next().unwrap().1);
            }
            if new_str == "true" {
                return (
                    Json::Boolean(true),
                    get_slice_from_breakpoint(str, Some(i + 4)),
                );
            }
        } else if c == 'f' {
            new_str.push(c);
            for _ in 0..4 {
                new_str.push(iter_str.next().unwrap().1);
            }
            if new_str == "false" {
                return (
                    Json::Boolean(false),
                    get_slice_from_breakpoint(str, Some(i + 5)),
                );
            }
        } else if c == 'n' {
            new_str.push(c);
            for _ in 0..3 {
                new_str.push(iter_str.next().unwrap().1);
            }
            if new_str == "null" {
                return (Json::Null, get_slice_from_breakpoint(str, Some(i + 4)));
            }
        } else if c == '[' {
            let mut vec = Vec::new();
            let mut str = Some(&str[1..]);
            'outer: loop {
                let (json_value, new_str) = parser(&str.unwrap());
                // println!("{:?}--{:?}--{:?}", vec, json_value, new_str);
                if json_value != Json::None {
                    vec.push(json_value);
                }
                let mut iter_new_str = new_str.unwrap().chars().enumerate();

                while let Some((i, c)) = iter_new_str.next() {
                    // Return if the next character is ']'
                    if c == ']' {
                        str = new_str;
                        if iter_new_str.next().is_some() {
                            break_point = Some(1);
                        } else {
                            break_point = None;
                        }
                        break 'outer;
                    }
                    // Skip ',' and ' ' characters
                    if c != ',' && c != ' ' {
                        str = Some(&new_str.unwrap()[i..]);
                        break;
                    }
                }
            }

            return (
                Json::Array(vec),
                get_slice_from_breakpoint(str.unwrap(), break_point),
            );
        } else if c == ']' {
            //Handling [] case
            return (Json::None, Some(str));
        } else if c == ' ' {
            continue;
        } else {
            panic!("Unexpected character: {}", c);
        }
    }
}

pub fn get_slice_from_breakpoint(str: &str, break_point: Option<usize>) -> Option<&str> {
    if break_point.is_none() {
        None
    } else {
        Some(&str[break_point.unwrap()..])
    }
}

#[derive(Debug, PartialEq)]
enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    None,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser1() {
        let str = r#""Hello world""#;
        assert_eq!(parser(str).0, Json::String("Hello world".to_string()));
    }
    #[test]
    fn test_parser2() {
        let str = "1024";
        assert_eq!(parser(str).0, Json::Number(1024 as f64));
    }

    #[test]
    fn test_parser3() {
        let str1 = "true";
        let str2 = "false";
        let str3 = "null";
        assert_eq!(parser(str1).0, Json::Boolean(true));
        assert_eq!(parser(str2).0, Json::Boolean(false));
        assert_eq!(parser(str3).0, Json::Null);
    }

    #[test]
    fn test_parser4() {
        let str1 = r#"[ true, false, 1024 , "Hello world" ]"#;
        let vec = vec![
            Json::Boolean(true),
            Json::Boolean(false),
            Json::Number(1024 as f64),
            Json::String("Hello world".to_string()),
        ];
        assert_eq!(parser(str1).0, Json::Array(vec));
    }
    #[test]
    fn test_parser5() {
        let str1 = r#"[true, false, [ 1024 ,null, "Hello world", []]]"#;
        let vec = vec![
            Json::Boolean(true),
            Json::Boolean(false),
            Json::Array(vec![
                Json::Number(1024 as f64),
                Json::Null,
                Json::String("Hello world".to_string()),
                Json::Array(vec![]),
            ]),
        ];
        assert_eq!(parser(str1).0, Json::Array(vec));
    }
}

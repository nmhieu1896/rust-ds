use std::collections::HashMap;

#[allow(dead_code)]
fn parser(str: &str) -> (Json, Option<&str>) {
    let mut iter_str = str.chars().enumerate();
    loop {
        let (_, c) = iter_str.next().unwrap();
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
                        break_point = Some(i + 1);
                        break;
                    }
                } else {
                    break_point = None;
                    break;
                }
            }
            return (
                Json::Number(new_str.parse::<f64>().unwrap()),
                if break_point.is_some() {
                    Some(&str[break_point.unwrap()..])
                } else {
                    None
                },
            );
        }
        if c == '"' {
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
                if break_point.is_some() {
                    Some(&str[break_point.unwrap()..])
                } else {
                    None
                },
            );
        }
        if c == 't' {
            new_str.push(c);
            for _ in 0..3 {
                new_str.push(iter_str.next().unwrap().1);
            }
            if new_str == "true" {
                return (Json::Boolean(true), Some(&str[4..]));
            }
        }
        if c == 'f' {
            new_str.push(c);
            for _ in 0..4 {
                new_str.push(iter_str.next().unwrap().1);
            }
            if new_str == "false" {
                return (Json::Boolean(false), Some(&str[5..]));
            }
        }
        // if c == '[' {
        //     let mut vec = Vec::new();
        //     loop {
        //         let next_c = iter_str.next().unwrap();
        //         if next_c == ']' {
        //             break;
        //         }
        //         if next_c == ',' {
        //             continue;
        //         }
        //         let json = parser(&next_c.to_string());
        //         vec.push(json);
        //     }
        //     return Json::Array(vec);
        // }
    }
}

#[derive(Debug, PartialEq)]
enum Json {
    Object(HashMap<String, Box<Json>>),
    Array(Vec<Json>),
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
        assert_eq!(parser(str1).0, Json::Boolean(true));
        assert_eq!(parser(str2).0, Json::Boolean(false));
    }

    // #[test]
    // fn test_parser4() {
    //     let str1 = r#"[true,false,1024,"Hello world"]"#;
    //     let vec = vec![
    //         Json::Boolean(true),
    //         Json::Boolean(false),
    //         Json::Number(1024 as f64),
    //         Json::String("Hello world".to_string()),
    //     ];
    //     assert_eq!(parser(str1).0, Json::Array(vec));
    // }
}

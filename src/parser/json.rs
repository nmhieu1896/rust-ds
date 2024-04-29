use std::collections::HashMap;

#[allow(dead_code)]
fn parser(str: String) -> Json {
    // str.chars().for_each(|c| {
    //     if c.is_digit(10) {
    //         return Json::Number(c.to_digit(10).unwrap() as f64);
    //     }
    //     if c.is_alphabetic() {
    //         return Json::String(c.to_string());
    //     }
    // });

    return Json::Boolean(false);
}

#[derive(Debug, PartialEq)]
enum Json {
    Object(HashMap<String, Json>),
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
        let str = r#"Hello world"#;
        assert_eq!(
            parser(str.to_string()),
            Json::String("Hello world".to_string())
        );
    }
    #[test]
    fn test_parser2() {
        let str = r#"1024"#;
        assert_eq!(parser(str.to_string()), Json::Number(1024 as f64));
    }
    // #[test]
    // fn test_parser1() {
    //   let str = r#"{ "name": "John", "age": 30, "car": null }"#;
    //   assert_eq!(parser(str.to_string()), Json::Object(HashMap::new()));
    // }
}

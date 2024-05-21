#[allow(dead_code)]
pub fn is_one_edit_distance(s: String, t: String) -> bool {
    let mut change = 0;
    let mut s1 = s.chars();
    let mut t1 = t.chars();
    let mut c1 = s1.next();
    let mut c2 = t1.next();

    loop {
        if c1.is_none() && c2.is_none() {
            break;
        }
        // one of the iterators is exhausted
        if (c1.is_none() && c2.is_some()) || (c1.is_some() && c2.is_none()) {
            change += 1;
            c2 = t1.next();
            c1 = s1.next();
            continue;
        }
        // equal
        if c1 == c2 {
            c2 = t1.next();
            c1 = s1.next();
            continue;
        }
        // not equal
        change += 1;
        // replace
        if s.len() == t.len() {
            c2 = t1.next();
            c1 = s1.next();
        }
        // delete
        if s.len() > t.len() {
            c1 = s1.next()
        }
        // insert
        if s.len() < t.len() {
            c2 = t1.next();
        }
    }

    return change == 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_edit_distance_insert() {
        let res = is_one_edit_distance("ab".to_string(), "acb".to_string());
        assert_eq!(res, true);
        let res = is_one_edit_distance("a".to_string(), "ac".to_string());
        assert_eq!(res, true);
        let res = is_one_edit_distance("ab".to_string(), "accb".to_string());
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_one_edit_distance_delete() {
        let res = is_one_edit_distance("a".to_string(), "".to_string());
        assert_eq!(res, true);
        let res = is_one_edit_distance("acb".to_string(), "ab".to_string());
        assert_eq!(res, true);
        let res = is_one_edit_distance("accb".to_string(), "ab".to_string());
        assert_eq!(res, false);
        let res = is_one_edit_distance("acbbcda".to_string(), "abbdad".to_string());
        assert_eq!(res, false);
    }
    #[test]
    fn test_is_one_edit_distance_replace() {
        let res = is_one_edit_distance("acb".to_string(), "adb".to_string());
        assert_eq!(res, true);
        let res = is_one_edit_distance("add".to_string(), "acb".to_string());
        assert_eq!(res, false);
    }
}

#[allow(dead_code)]
pub fn is_one_edit_distance(s: String, t: String) -> bool {
    if s.len() > t.len() {
        return is_one_edit_distance(t, s);
    }
    if t.len() - s.len() > 1 {
        return false;
    }

    let mut change = 0;
    let mut s_iter = s.chars();
    let mut t_iter = t.chars();
    let mut c_s = s_iter.next();
    let mut c_t = t_iter.next();

    loop {
        if c_s.is_none() && c_t.is_none() {
            break;
        }
        // equal, both are Some
        if c_s == c_t {
            c_t = t_iter.next();
            c_s = s_iter.next();
            continue;
        }
        // not equal
        change += 1;

        // one is none (insert) or same length
        if c_t.is_none() || (s.len() == t.len()) {
            c_s = s_iter.next();
            c_t = t_iter.next();
            continue;
        }

        c_t = t_iter.next();
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

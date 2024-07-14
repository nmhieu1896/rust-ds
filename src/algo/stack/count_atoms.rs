#[allow(dead_code)]
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let bytes = formula.as_bytes();
        let (mut vec, _) = Self::count(bytes, 0);

        let mut res = String::new();
        vec.sort_by(|a, b| a.0.cmp(&b.0));

        let mut i = 0;
        while i < vec.len() {
            let mut count = 0;
            //while the element is not the last in vec
            while vec.get(i + 1).is_some() && vec[i].0 == vec[i + 1].0 {
                count += vec[i].1;
                i += 1;
            }
            count += vec[i].1;
            res.push_str(&vec[i].0);
            if count > 1 {
                res.push_str(&count.to_string());
            }
            i += 1;
        }
        return res;
    }

    pub fn count(f: &[u8], mut i: usize) -> (Vec<(String, usize)>, usize) {
        let mut vec = Vec::new();
        while i < f.len() {
            match f[i] {
                b'A'..=b'Z' => vec.push(((f[i] as char).to_string(), 1)),
                b'a'..=b'z' => vec.last_mut().unwrap().0.push(f[i] as char),
                b'0'..=b'9' => {
                    let (num, new_i) = Self::get_number(f, i);
                    i = new_i;
                    vec.last_mut().unwrap().1 = num;
                }
                b'(' => {
                    let (res, new_i) = Self::count(f, i + 1);
                    i = new_i;
                    vec.extend(res);
                }
                b')' => {
                    let (mult, new_i) = Self::get_number(f, i + 1);
                    i = new_i;
                    vec.iter_mut().for_each(|(_, v)| *v *= mult);
                    return (vec, i);
                }
                _ => {}
            }
            i += 1;
        }
        (vec, i)
    }

    //stop at the last digit.
    pub fn get_number(f: &[u8], i: usize) -> (usize, usize) {
        let mut i = i;
        if f.get(i).is_none() || !f[i].is_ascii_digit() {
            return (1, i - 1);
        }
        let mut num = (f[i] - b'0') as usize;
        while i + 1 < f.len() && f[i + 1].is_ascii_digit() {
            num = num * 10 + (f[i + 1] - b'0') as usize;
            i += 1;
        }
        return (num, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_of_atoms("H2O".to_string()),
            "H2O".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_of_atoms("Mg(OH)2".to_string()),
            "H2MgO2".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
            "K4N2O14S4".to_string()
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_of_atoms("Mg(H2O)N".to_string()),
            "H2MgNO".to_string()
        );
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::count_of_atoms("Mg(H2O)".to_string()),
            "H2MgO".to_string()
        );
    }
}

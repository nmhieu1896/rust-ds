pub fn _run() {
    let res = _int_to_roman(123);
    println!("123: {}", res);
    let res = _int_to_roman(58);
    println!("58: {}", res);
    let res = _int_to_roman(1994);
    println!("1994: {}", res);
    let res = _int_to_roman(2004);
    println!("2004: {}", res);
}
const _ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const _TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const _HUNDS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const _THOUDS: [&str; 4] = ["", "M", "MM", "MMM"];

pub fn _int_to_roman(num: i32) -> String {
    format!(
        "{}{}{}{}",
        _THOUDS[(num / 1000 % 10) as usize],
        _HUNDS[(num / 100 % 10) as usize],
        _TENS[(num / 10 % 10) as usize],
        _ONES[(num % 10) as usize]
    )
}

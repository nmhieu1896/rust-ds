pub fn _run() {
    let mut number: i32 = 7;
    let count = number.count_ones();
    println!("count: {}", count);

    let rightmost_bit_set = (number & 1) != 0;
    println!("rightmost_bit_set: {}", rightmost_bit_set);

    // number = number & number.wrapping_sub(1);
    // println!("After setting rightmost bit to zero: {}", number);
    number = (number - 1) | number;
    println!("After moving rightmost bit to the right: {}", number);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let x: u8 = 0b1010_1010;
        let y = !x;
        println!("x: {:b}", x);
        println!("y: {:b}", y);
        let count = x.count_ones();
        println!("{}", count); // Outputs: 4
        let odd = 7;
        let is_odd = (odd & 1) == 1;
        println!("{}", is_odd); // Outputs: 3
    }
}

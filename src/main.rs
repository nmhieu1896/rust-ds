mod fenwick_tree;
mod hash;
mod heap;

fn main() {
    // heap::_run();
    // hash::group_anagrams::_run();
    // hash::four_sum_2::_run();
    // hash::int_to_roman::_run();
    // let mut number: i32 = 7;

    fenwick_tree::_run();
    // let count = number.count_ones();
    // println!("count: {}", count);

    // let rightmost_bit_set = (number & 1) != 0;
    // println!("rightmost_bit_set: {}", rightmost_bit_set);

    // // number = number & number.wrapping_sub(1);
    // // println!("After setting rightmost bit to zero: {}", number);
    // number = (number - 1) | number;
    // println!("After moving rightmost bit to the right: {}", number);
}

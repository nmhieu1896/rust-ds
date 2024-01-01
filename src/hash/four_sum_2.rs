use std::{collections::HashMap, vec};

pub fn _run() {
    let v1 = vec![1, 2];
    let v2 = vec![-2, -1];
    let v3 = vec![-1, 2];
    let v4 = vec![0, 2];

    let count = _four_sum_count(v1, v2, v3, v4);
    println!("count {}", count);
}

pub fn _four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let mut map1 = HashMap::<i32, i32>::new();

    for &item_v1 in nums1.iter() {
        for &item_v2 in nums2.iter() {
            *map1.entry(item_v1 + item_v2).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for &item_v3 in nums3.iter() {
        for &item_v4 in nums4.iter() {
            total += map1.get(&(-item_v3 - item_v4)).unwrap_or(&0)
        }
    }

    return total;
}

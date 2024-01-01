use std::vec;

pub fn _run() {
    println!("Hello, world!");
    // let arr = vec![10, 4, 20, 16, 14, 8, 1, 12, 24, 9, 18];
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 1];
    _test_heap(arr.clone());
    _find_kth_smallest(arr, 5)
}

pub fn _test_heap(arr: Vec<i32>) {
    let mut heap = _heapify(arr);
    println!("heap: {:?}", heap);
    _print_heap(&heap);
    // println!("heap: {:?}", heap);

    // _add_to_heap(&mut heap, 2);
    // println!("heap: {:?}", heap);
    // _add_to_heap(&mut heap, 3);
    // println!("heap: {:?}", heap);
    // _shift(&mut heap);
    // println!("heap: {:?}", heap);
    _shift(&mut heap);
    // println!("heap: {:?}", heap);
}

fn _add_to_heap(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);

    _heapify_up(vec, vec.len() - 1);
}

fn _print_heap(vec: &Vec<i32>) {
    //         |--------------1                  || 14 n               => 15
    //         |----------/-------\              || 10 / 7 \           => 19
    //         |------2-------|-------3          || 6 n 15 n           => 23
    //         |----/---\-----------/---\        || 4 / 3 \ 11 / 3 \   => 25
    //         |--4-------5-------6-------7      || 2 n 7 n 7 n 7 n    => 27
    //         |-/-\-----/-\-----/-\-----/-\     || 1 / 1 \ 5  1       => 28
    //         |7---8---7---9---1---1---1---1    || 0 n 3 n 3 n 3 n    => 29
    // cal num_lines by log 2 of vec.len()
    let mut num_lines = (f64::log2(vec.len() as f64) as usize) + 1;
    let all_lines = num_lines * 2 - 1;
    let num_space = 1; // space of a number
    let arrow_space = 1; // space of an arrow
    let base_space = 3;
    let mut idx = 0;
    let character = "-";
    let base: u32 = 2;
    //-------------
    for i in 0..all_lines {
        if idx >= vec.len() {
            break;
        }

        if i % 2 == 1 {
            let mut count = 0;
            let mut repeat = if i == all_lines - 2 {
                1
            } else {
                1 * 3 + num_space
            };

            while i < all_lines - 2 && count < num_lines - 2 {
                repeat = (repeat - num_space) * 3 + num_space;
                count = count + 1;
            }

            let indent_str = character.repeat(repeat);
            println!("{}/", indent_str);
            // let mut arrows = Vec::new();
            // for j in 0..(i * 2) {
            //     if j % 2 == 0 {
            //         arrows.push("/");
            //     } else {
            //         arrows.push("\\");
            //     }
            // }
            // println!(
            //     "{}{}",
            //     character.repeat(all_lines - 2 * i + 1),
            //     arrows.join("-")
            // );
            continue;
        }

        let indent_str = character.repeat((base.pow(num_lines as u32) - 2) as usize);
        let mut content = format!("{}{}", indent_str, vec[idx]);
        idx = idx + 1;

        // Print nodes
        for _ in 0..base.pow((i / 2) as u32) - 1 {
            let mut count = 0;
            let mut repeat = if num_lines == 1 {
                base_space
            } else {
                base_space + 2 * num_space + 2 * arrow_space
            }; //
               // println!("repeat: {} || num_lines: {}", repeat, num_lines);
            while num_lines >= 2 && count < num_lines - 2 {
                repeat = repeat * 2 + num_space;
                count = count + 1;
            }
            if idx < vec.len() {
                content =
                    content + character.repeat(repeat as usize).as_ref() + &vec[idx].to_string();
                idx = idx + 1;
            }
        }
        println!("{}", content);
        num_lines = num_lines - 1;
    }
}

fn _shift(vec: &mut Vec<i32>) {
    let len = vec.len();
    vec.swap(0, len - 1);
    vec.remove(len - 1);

    _heapify_down(vec, 0);
}

fn _heapify(arr: Vec<i32>) -> Vec<i32> {
    let mut heap = arr.clone();
    for i in 1..heap.len() {
        _heapify_up(&mut heap, i);
    }

    return heap;
}

fn _heapify_up(heap: &mut Vec<i32>, i: usize) {
    let mut i = i;
    while i > 1 {
        let parent = (i - 1) / 2;
        if heap[parent] > heap[i] {
            heap.swap(parent, i);
        }
        i = parent;
    }
}

fn _heapify_down(heap: &mut Vec<i32>, i: usize) {
    let mut i = i;
    while i < heap.len() {
        let left = i * 2;
        let right = i * 2 + 1;
        let mut min = i;
        if left < heap.len() && heap[left] < heap[min] {
            min = left;
        }
        if right < heap.len() && heap[right] < heap[min] {
            min = right;
        }
        if min == i {
            break;
        }
        heap.swap(i, min);
        i = min;
    }
}

fn _heapify_down_v2(heap: &mut Vec<i32>, i: usize) {
    let mut i = i;
    while i < heap.len() - 1 {
        let left = i * 2 + 1;
        let right = i * 2 + 2;

        if left < heap.len() && heap[i] > heap[left] {
            if right < heap.len() && heap[right] < heap[left] {
                heap.swap(i, right);
                i = right;
            } else {
                heap.swap(i, left);
                i = left;
            }
        } else if right < heap.len() && heap[i] > heap[right] {
            if heap[left] > heap[right] {
                heap.swap(i, right);
                i = right;
            } else {
                heap.swap(i, left);
                i = left;
            }
        } else {
            break;
        }
    }
}

fn _find_kth_smallest(arr: Vec<i32>, k: usize) {
    let mut heap = _heapify(arr);
    println!("heap: {:?}", heap);
    for _ in 0..k - 1 {
        _shift(&mut heap);
    }
    println!("Kth smallest: {:?}", heap[0]);
}

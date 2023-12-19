use std::vec;

pub fn _run() {
    println!("Hello, world!");
    let arr = vec![10, 4, 20, 16, 14, 8, 1, 12, 24, 9, 18];
    let mut heap = heapify(arr);

    println!("heap: {:?}", heap);

    add_to_heap(&mut heap, 2);
    println!("heap: {:?}", heap);
    add_to_heap(&mut heap, 3);
    println!("heap: {:?}", heap);
    shift(&mut heap);
    println!("heap: {:?}", heap);
    shift(&mut heap);
    println!("heap: {:?}", heap);
}

fn add_to_heap(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);

    heapify_up(vec, vec.len() - 1);
}

fn shift(vec: &mut Vec<i32>) {
    let len = vec.len();
    vec.swap(0, len - 1);
    vec.remove(len - 1);

    heapify_down(vec, 0);
}

fn heapify(arr: Vec<i32>) -> Vec<i32> {
    let mut heap = arr.clone();
    for i in 1..heap.len() {
        heapify_up(&mut heap, i);
    }

    return heap;
}

fn heapify_up(heap: &mut Vec<i32>, i: usize) {
    let mut i = i;
    while i > 1 {
        let parent = (i - 1) / 2;
        if heap[parent] > heap[i] {
            heap.swap(parent, i);
        }
        i = parent;
    }
}

fn heapify_down(heap: &mut Vec<i32>, i: usize) {
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

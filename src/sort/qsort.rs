pub fn _run() {
    let mut x = vec![
        43, 1232, 12, 42, 67, 9, 20, 753, 58, 24, 57, 87, 5, 23, 46, 19, 46, 123, 41,
    ];
    qsort(&mut x);

    println!("{:?}", x)
}

#[allow(dead_code)]
fn qsort(vec: &mut [i32]) {
    //Handling base case for length <2
    if vec.len() < 2 {
        return;
    }

    // pick last item as pivot
    let last_idx = vec.len() - 1;
    let pivot = vec[last_idx];
    // ptr is to count the number of item that smaller than pivot
    // type of ptr is "usize", which is >0, so that can not use -1 as init value
    // So I'm naively pick out-of-range as an init value
    let mut ptr = last_idx + 1;

    for i in 0..last_idx {
        if vec[i] < pivot {
            ptr = if ptr == vec.len() { 0 } else { ptr + 1 };
            if vec[i] < vec[ptr] {
                vec.swap(i, ptr);
            }
            continue;
        }
    }

    // if ptr is still the init value (similar to -1), set it to 0
    ptr = if ptr == last_idx + 1 { 0 } else { ptr + 1 };
    vec.swap(ptr, last_idx);

    qsort(&mut vec[0..ptr]);
    qsort(&mut vec[ptr + 1..]);
}

#[cfg(test)]
mod test_qsort {
    use super::*;

    #[test]
    fn test_qsort1() {
        let mut x = vec![
            43, 1232, 12, 42, 67, 9, 20, 753, 58, 24, 57, 87, 5, 23, 46, 19, 46, 123, 41,
        ];
        qsort(&mut x);
        assert_eq!(
            x,
            vec![5, 9, 12, 19, 20, 23, 24, 41, 42, 43, 46, 46, 57, 58, 67, 87, 123, 753, 1232]
        );
    }
    #[test]
    fn test_qsort2() {
        let mut x = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        qsort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_qsort3() {
        let mut x = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        qsort(&mut x);
        assert_eq!(x, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    // test vec of random chaos vector with negative number
    #[test]
    fn test_qsort4() {
        let mut x = vec![
            43, 1232, 12, 42, 67, 9, 20, 753, 58, 24, 57, 87, 5, 23, 46, 19, 46, 123, 41, -1, -2,
            -3, -4, -5, -6, -7, -8, -9,
        ];
        qsort(&mut x);
        assert_eq!(
            x,
            vec![
                -9, -8, -7, -6, -5, -4, -3, -2, -1, 5, 9, 12, 19, 20, 23, 24, 41, 42, 43, 46, 46,
                57, 58, 67, 87, 123, 753, 1232
            ]
        );
    }
}

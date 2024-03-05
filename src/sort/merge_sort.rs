#[allow(dead_code)]
fn merge_sort(vec: &mut Vec<i32>) {
    if vec.len() < 2 {
        return;
    }

    let mid = vec.len() / 2;
    let mut left = vec[0..mid].to_vec();
    let mut right = vec[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            vec[k] = left[i];
            i += 1;
        } else {
            vec[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        vec[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        vec[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut vec = vec![3, 2, 1, 5, 4];
        merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5])
    }
    #[test]
    fn test_merge_sort2() {
        let mut vec = vec![
            43, 1232, 12, 42, 67, 9, 20, 753, 58, 24, 57, 87, 5, 23, 46, 19, 46, 123, 41, -1, -2,
            -3, -4, -5, -6, -7, -8, -9,
        ];
        merge_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                -9, -8, -7, -6, -5, -4, -3, -2, -1, 5, 9, 12, 19, 20, 23, 24, 41, 42, 43, 46, 46,
                57, 58, 67, 87, 123, 753, 1232
            ]
        )
    }
}

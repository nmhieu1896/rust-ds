#[allow(dead_code)]
pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let n = arr.len();
    let mut low = 0.0;
    let mut high = 1.0;
    let mut res = vec![0, 1];
    while high - low > 1e-9 {
        let mid = (low + high) / 2.0;
        let mut count = 0;
        let mut j = 1;
        let mut max_fraction = 0.0;
        for i in 0..n {
            while j < n && arr[i] as f64 > arr[j] as f64 * mid {
                j += 1;
            }
            count += n - j;
            if j < n {
                let fraction = arr[i] as f64 / arr[j] as f64;
                if fraction > max_fraction {
                    max_fraction = fraction;
                    res = vec![arr[i], arr[j]];
                }
            }
        }
        if k == count as i32 {
            return res;
        } else if k >= count as i32 {
            low = mid;
        } else {
            high = mid;
        }
    }
    res
}

// pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
//     // impl with O(N^2) time complexity
//     let mut vec: Vec<(f64, (usize, usize))> = vec![];
//     for i in 0..arr.len() {
//         for j in i + 1..arr.len() {
//             vec.push((arr[i] as f64 / arr[j] as f64, (i, j)));
//         }
//     }
//     vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
//     let nth = vec.get(k as usize - 1).unwrap();
//     return vec![arr[nth.1 .0], arr[nth.1 .1]];
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest_prime_fraction() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let res = kth_smallest_prime_fraction(arr, k);
        assert_eq!(res, vec![2, 5]);
    }

    #[test]
    fn test_kth_smallest_prime_fraction2() {
        let arr = vec![1, 7];
        let k = 1;
        let res = kth_smallest_prime_fraction(arr, k);
        assert_eq!(res, vec![1, 7]);
    }
}

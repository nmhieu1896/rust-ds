#[allow(dead_code)]
pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    let mut dist = i32::MIN;
    arrays.iter().for_each(|arr| {
        if max > i32::MIN && min < i32::MAX {
            dist = dist.max(arr[arr.len() - 1] - min).max(max - arr[0]);
        }
        // if arr[arr.len() - 1] > max {
        //     max = arr[arr.len() - 1];
        //     // dist = dist.max(arr[arr.len() - 1] - min);
        //   } else if arr[0] < min {
        //     // dist = dist.max(max - arr[0]);
        //     min = arr[0];
        //   }
        max = max.max(arr[arr.len() - 1]);
        min = min.min(arr[0]);
        // dist = dist.max(max - min);
    });
    return dist;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance() {
        let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
        let res = max_distance(arrays);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_max_distance2() {
        let arrays = vec![vec![1], vec![1]];
        let res = max_distance(arrays);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_max_distance3() {
        let arrays = vec![vec![1, 4], vec![0, 6]];
        let res = max_distance(arrays);
        assert_eq!(res, 5);
    }
    #[test]
    fn test_max_distance4() {
        let arrays = vec![vec![3, 4], vec![0, 6]];
        let res = max_distance(arrays);
        assert_eq!(res, 4);
    }
}

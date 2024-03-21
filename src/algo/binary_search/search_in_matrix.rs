#[allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.len() == 0 {
        return false;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let len = rows * cols;
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        let mid = (left + right) / 2;

        if matrix[mid / cols][mid % cols] < target {
            left = mid + 1
        } else {
            right = mid;
        }
    }

    return if matrix[left / cols][left % cols] == target {
        true
    } else {
        false
    };
}

#[cfg(test)]
mod test {
    use super::search_matrix;

    #[test]
    fn test_search_matrix1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert_eq!(search_matrix(matrix, target), true);
    }

    #[test]
    fn test_search_matrix2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert_eq!(search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix3() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 60;
        assert_eq!(search_matrix(matrix, target), true);
    }

    #[test]
    fn test_search_matrix4() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 0;
        assert_eq!(search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix5() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 61;
        assert_eq!(search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix6() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 2;
        assert_eq!(search_matrix(matrix, target), false);
    }

    #[test]
    fn test_search_matrix7() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 20;
        assert_eq!(search_matrix(matrix, target), true);
    }
}

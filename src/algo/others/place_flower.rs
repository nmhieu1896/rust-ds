#[allow(dead_code)]
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut n = n;
    let mut vec = flowerbed;

    for i in 0..vec.len() {
        if i == 0 {
            if vec[i] == 0 && vec.len() == 1 {
                n -= 1;
                vec[i] = 1;
            } else if vec[i] == 0 && vec[i + 1] == 0 {
                n -= 1;
                vec[i] = 1;
            }
        } else if i == vec.len() - 1 {
            if vec[i] == 0 && vec[i - 1] == 0 {
                n -= 1;
                vec[i] = 1;
            }
        } else if vec[i - 1] == 0 && vec[i] == 0 && vec[i + 1] == 0 {
            n -= 1;
            vec[i] = 1;
        }
    }

    return n <= 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_place_flowers1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_can_place_flowers2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), false);
    }

    #[test]
    fn test_can_place_flowers3() {
        let flowerbed = vec![0, 0, 1, 0, 0];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_can_place_flowers4() {
        let flowerbed = vec![0, 0, 1, 0, 0];
        let n = 2;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_can_place_flowers5() {
        let flowerbed = vec![0, 0, 0, 0, 0];
        let n = 3;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_can_place_flowers6() {
        let flowerbed = vec![0];
        let n = 1;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }

    #[test]
    fn test_can_place_flowers7() {
        let flowerbed = vec![1];
        let n = 0;
        assert_eq!(can_place_flowers(flowerbed, n), true);
    }
}

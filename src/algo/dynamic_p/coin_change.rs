#[allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let res = dp(&coins, amount);
    if res == i32::MAX {
        return -1;
    }
    return res;
}

pub fn dp(coins: &Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    if amount < 0 {
        return -1;
    }
    let mut min_val = i32::MAX;

    coins.iter().for_each(|&coin| {
        if amount >= coin {
            let res = dp(coins, amount - coin);
            if res >= 0 && res < i32::MAX {
                min_val = min_val.min(1 + res);
            }
        }
    });
    return min_val;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coin_change1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(coin_change(coins, amount), 3);
    }

    #[test]
    fn test_coin_change2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(coin_change(coins, amount), -1);
    }

    #[test]
    fn test_coin_change3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(coin_change(coins, amount), 0);
    }

    #[test]
    fn test_coin_change4() {
        let coins = vec![1];
        let amount = 1;
        assert_eq!(coin_change(coins, amount), 1);
    }

    #[test]
    fn test_coin_change5() {
        let coins = vec![1, 4, 5];
        let amount = 13;
        assert_eq!(coin_change(coins, amount), 3);
    }
}

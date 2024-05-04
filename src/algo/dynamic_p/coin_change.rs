#[allow(dead_code)]
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let iter = coins.iter().map(|&x| x as usize);
    let amount = amount as usize;
    let mut map = vec![amount + 1; amount + 1];
    map[0] = 0;

    for coin in iter {
        for i in coin..=amount {
            map[i] = map[i].min(map[i - coin] + 1);
        }
    }
    if map[amount] > amount {
        -1
    } else {
        map[amount] as i32
    }
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

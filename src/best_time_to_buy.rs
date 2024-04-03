use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    for i in 1..prices.len() {
        let daily_profit = prices[i] - prices[i-1];
        if daily_profit > 0 {
            profit = profit + daily_profit;
        }
    }

    profit
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let prices = vec![7,1,5,3,6,4];
        assert_eq!(max_profit(prices),  7);
    }
}
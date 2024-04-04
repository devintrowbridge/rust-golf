// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
// On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.
// Find and return the maximum profit you can achieve.

// Example 1:
// Input: prices = [7,1,5,3,6,4]
// Output: 7
// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
// Total profit is 4 + 3 = 7.


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
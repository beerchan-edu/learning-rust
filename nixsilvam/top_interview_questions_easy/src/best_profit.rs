// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
pub fn best_profit() {

}

fn finding_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for price in 0..(prices.len()) {
        if prices[price] < prices[price + 1] {
            profit += prices[price];
            profit += prices[price + 1] - prices[price];
        }
    }
    profit
}

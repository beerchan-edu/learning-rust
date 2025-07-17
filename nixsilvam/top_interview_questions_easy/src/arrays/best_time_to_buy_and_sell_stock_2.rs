// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
pub fn best_profit() {
let prices_1: Vec<i32> = vec![7,1,5,3,6,4];
let prices_2: Vec<i32> = vec![1,2,3,4,5];
let prices_3: Vec<i32> = vec![7,6,4,3,1];

println!("{}", finding_profit(prices_1));
println!("{}", finding_profit(prices_2));
println!("{}", finding_profit(prices_3));

}

fn finding_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for price in 0..(prices.len() - 1) {
        if prices[price] < prices[price + 1] {
            profit += prices[price + 1] - prices[price];
        }
    }
    profit
}

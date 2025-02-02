use std::collections::HashMap;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut pointer = 0;
    for i in 1..nums.len() {
        if nums.get(i) == nums.get(pointer) {
            continue;
        }
        pointer += 1;
        nums[pointer] = nums[i];
    }
    pointer += 1;
    pointer as i32
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }
    return profit;
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut helper = nums.clone();
    let n = nums.len() as i32;
    let k = k % n;
    for i in 0..nums.len() {
        let shifted_index = (n + i as i32 - k) % n;
        nums[i] = helper[shifted_index as usize];
    }
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashmap: HashMap<i32, bool> = HashMap::new();
    for i in nums {
        if hashmap.contains_key(&i) {
            return true;
        }
        hashmap.insert(i, true);
    }
    return false;
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in nums {
        result = result ^ i;
    }
    return result;
}

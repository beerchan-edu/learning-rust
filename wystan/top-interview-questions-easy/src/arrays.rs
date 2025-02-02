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

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let nums_map_1 = to_hash_map(&nums1);
    let nums_map_2 = to_hash_map(&nums2);
    let mut result: Vec<i32> = Vec::new();
    for (k, v1) in nums_map_1.iter() {
        match nums_map_2.get(k) {
            None => continue,
            Some(v2) => {
                let n = std::cmp::min(v1, v2);
                for i in 0..(*n) {
                    result.push(*k);
                }
            }
        }
    }
    return result;
}

pub fn to_hash_map(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut result: HashMap<i32, i32> = HashMap::new();
    for i in nums {
        let count = result.entry(*i).or_insert(0);
        *count += 1;
    }
    result
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = digits.clone();
    for (i, elem) in digits.iter().enumerate().rev() {
        if *elem == 9 {
            result[i] = 0;
            continue;
        }
        result[i] += 1;
        return result;
    }
    result.insert(0, 1);
    return result;
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut pointer = 0;
    let n = nums.len();
    for i in 0..n {
        if nums[i] != 0 {
            nums[pointer] = nums[i];
            pointer += 1;
        }
    }
    for i in pointer..nums.len() {
        nums[i] = 0;
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    for (i, &elem) in nums.iter().enumerate() {
        hashmap.insert(elem, i);
    }
    for (i, &elem) in nums.iter().enumerate() {
        match hashmap.get(&(target - elem)) {
            None => continue,
            Some(&j) => {
                if i == j {
                    continue;
                }
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0, 0]
}

pub fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..n / 2 {
        for j in i..n - i - 1 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[n - 1 - j][i];
            matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
            matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
            matrix[j][n - 1 - i] = temp;
        }
    }
}

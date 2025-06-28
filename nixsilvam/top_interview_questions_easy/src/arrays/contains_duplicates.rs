// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/578/
use std::collections::HashSet;

pub fn solution_contains_duplicates() {
    let nums: Vec<i32> = vec![1,1,1,3,3,4,3,2,4,2];
    print!("{:?}", duplicates(nums));
}

fn duplicates(nums: Vec<i32>) -> bool {
    let mut counts = HashSet::new();
    for num in nums {
        if !counts.insert(num) {
            return true;
        }
    }
    return false;
}
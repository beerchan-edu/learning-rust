// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/546/

use std::collections::HashMap;

pub fn solution_two_sums() {
    let nums1= vec![2,7,11,15];
    let target1 = 9;
    let nums2 = vec![3,2,4];
    let target2 =6;
    let nums3 = vec![3,3];
    let target3 = 6;

    let nums4= vec![2,7,11,15];
    let target4 = 9;
    let nums5 = vec![3,2,4];
    let target5 =6;
    let nums6 = vec![3,3];
    let target6 = 6;

    println!("{:?}", two_sums_bruteforce(nums1, target1));
    println!("{:?}", two_sums_bruteforce(nums2, target2));
    println!("{:?}", two_sums_bruteforce(nums3, target3));

    println!("{:?}", two_sums_hashmap(nums4, target4));
    println!("{:?}", two_sums_hashmap(nums5, target5));
    println!("{:?}", two_sums_hashmap(nums6, target6));
}

fn two_sums_bruteforce(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for first in 0..nums.len() {
        for second in first + 1..nums.len() {
            if nums[first] + nums[second] == target {
                return vec![first as i32, second as i32]
            }
        }
    }
    return vec![] // this won't ever be reached due to the problem constraints
}

fn two_sums_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32]
        }
        map.insert(*num, i);
    }
    return vec![] // this won't ever be reached due to the problem constraints
}
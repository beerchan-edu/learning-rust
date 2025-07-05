// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/546/

pub fn solution_two_sums() {
    let nums1= vec![2,7,11,15];
    let target1 = 9;
    let nums2 = vec![3,2,4];
    let target2 =6;
    let nums3 = vec![3,3];
    let target3 = 6;

    println!("{:?}", two_sums_bruteforce(nums1, target1));
    println!("{:?}", two_sums_bruteforce(nums2, target2));
    println!("{:?}", two_sums_bruteforce(nums3, target3));
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

//hashmap
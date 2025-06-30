// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/549/

pub fn solution_single_numbers() {
    let nums_1: Vec<i32> = vec![2,2,1];
    let nums_2: Vec<i32> = vec![4,1,2,1,2];
    let nums_3: Vec<i32> = vec![1];
    println!("{}", single_numbers(nums_1));
    println!("{}", single_numbers(nums_2));
    println!("{}", single_numbers(nums_3))
}

pub fn single_numbers(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for num in nums {
        result ^= num;
    }
    result
}

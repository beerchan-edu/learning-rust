// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/567/

pub fn solution_move_zeroes() {
    let nums1: Vec<i32> = vec![0,1,0,3,12];
    let nums2 : Vec<i32> = vec![0];

    println!("{:?}", move_zeroes1(nums1));
    println!("{:?}", move_zeroes1(nums2));

}

fn move_zeroes1(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut zeroes = 0;
    for num in 0..(nums.len()) {
        if nums[num] != 0 {
            result.push(nums[num]);  
            
        } else {
            zeroes += 1       
        }
    }

    for _ in 0..zeroes {
        result.push(0);
    }

    result
}
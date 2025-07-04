// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/567/

pub fn solution_move_zeroes() {
    let mut nums1: Vec<i32> = vec![0,1,0,3,12];
    let mut nums2 : Vec<i32> = vec![0];

    println!("{:?}", move_zeroes1(&nums1));
    println!("{:?}", move_zeroes1(&nums2));

    println!("{:?}", move_zeroes2(&mut nums1));
    println!("{:?}", move_zeroes2(&mut nums2));
}

fn move_zeroes1(nums: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut zeroes = 0;
    for num in 0..nums.len() {
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

fn move_zeroes2 (nums: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut write = 0;

    for read in 0..nums.len() {
        if nums[read] != 0 {
            nums[write] = nums[read];
            write +=1 
        }
    }

    for i in write..nums.len() {
        nums[i] = 0;
    }

    nums
}
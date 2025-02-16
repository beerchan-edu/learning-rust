// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/

pub fn solution_remove_duplicates() {
    let mut nums1 = vec![1,1,2];
    let mut nums2 = vec![0,0,1,1,1,2,2,3,3,4];
    println!("{}", remove_duplicates(&mut nums1));
    println!("{}", remove_duplicates(&mut nums2));
}


fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 1;
    while i < nums.len() {
        if nums[i] == nums[i - 1] {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    nums.len() as i32
}

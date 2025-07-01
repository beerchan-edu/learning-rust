// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/

use std::collections::HashMap;

pub fn solution_intersection () {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    let nums3 = vec![4,9,5];
    let nums4 = vec![9,4,9,8,4];
    print!("{:?}", intersect(nums1, nums2));
    print!("{:?}", intersect(nums3, nums4));

}

fn intersect (nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut intersection: Vec<i32> = Vec::new();
    let mut map = HashMap::new();

    for num in nums1 {
        let count = map.entry(num).or_insert(0);
        *count += 1
    }

    for num in nums2 {
        if let Some(count) = map.get_mut(&num) {
            intersection.push(num);
            *count -= 1;
            if *count == 0 {
                map.remove(&num);
            }
        }
    }

    intersection

}

fn intersect_sorted (nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    
}
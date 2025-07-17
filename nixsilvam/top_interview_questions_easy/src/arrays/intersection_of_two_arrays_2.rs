// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/

use std::collections::HashMap;

pub fn solution_intersection () {
    let nums1 = vec![1,2,2,1];
    let nums2 = vec![2,2];
    let nums3 = vec![4,9,5];
    let nums4 = vec![9,4,9,8,4];
    let nums5 = vec![1, 2, 2, 3];
    let nums6 = vec![2, 2, 4];
    let nums7 = vec![1,2,2,1];
    let nums8 = vec![2,2];
    let nums9 = vec![4,9,5];
    let nums10 = vec![9,4,9,8,4];

    print!("{:?}", intersect(nums1, nums2));
    print!("{:?}", intersect(nums3, nums4));
    print!("{:?}", intersect_sorted(nums5, nums6));
    print!("{:?}", intersect_chunks(nums7, nums8));
    print!("{:?}", intersect_chunks(nums9, nums10));

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
    let mut intersect: Vec<i32> = Vec::new();
    if !nums1.is_sorted() && !nums2.is_sorted() {
        return intersect
    }

    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            intersect.push(nums1[i]);
            i += 1;
            j += 1;
        }
        else if nums1[i] > nums2[j] {
            j += 1;
        }
        else {
            i += 1;
        }
    }
    intersect
}

fn intersect_chunks (nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut intersection: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    let chunk_size = 3;

    for num in nums1 {
        let count = map.entry(num).or_insert(0);
        *count += 1
    }

    for chunk in nums2.chunks(chunk_size) {
        for &num in chunk {
            if let Some(count) = map.get_mut(&num) {
                intersection.push(num);
                *count -= 1;
                if *count == 0 {
                    map.remove(&num);
                }
            }
        }
    }

    intersection

    
}
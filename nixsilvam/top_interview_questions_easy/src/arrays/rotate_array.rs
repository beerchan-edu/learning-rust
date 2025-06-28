// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/646/


pub fn solution_rotate() {
    let mut nums_1 = vec![1,2,3,4,5,6,7];
    let mut nums_2 = vec![-1,-100,3,99];
    println!("{:?}", (rotate_1(&mut nums_1, 3)));
    println!("{:?}", (rotate_1(&mut nums_2, 2)));
    println!("{:?}", (rotate_2(&mut nums_1, 3)));
    println!("{:?}", (rotate_2(&mut nums_2, 2)));
    println!("{:?}", (rotate_3(&mut nums_1, 3)));
    println!("{:?}", (rotate_3(&mut nums_2, 2)));
}

fn rotate_1(nums: &mut Vec<i32>, k: u32) -> &mut Vec<i32> { 
    let mut step = 0;
    while step < k {
        let last_element = nums.pop();
        match last_element {
            Some(i) => nums.insert(0, i),
            _ => print!("There are no elements in vector")
        }
        step += 1
    }
    nums
}

fn rotate_2(nums: &mut Vec<i32>, k: u32) -> &mut Vec<i32> {
    let steps: usize = k as usize;
    nums.rotate_right(steps);
    nums
}

fn rotate_3(nums: &mut Vec<i32>, k: u32) -> &mut Vec<i32> {
    let len = nums.len();
    let k = (k as usize) % len;
    if k == 0 {return nums;}

    nums[..len - k].reverse();
    nums[len - k..].reverse();
    nums.reverse();

    nums
}

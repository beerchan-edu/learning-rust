pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut pointer = 0;
    for i in 1..nums.len() {
        if nums.get(i) == nums.get(pointer) {
            continue;
        }
        pointer += 1;
        nums[pointer] = nums[i];
    }
    pointer += 1;
    pointer as i32
}

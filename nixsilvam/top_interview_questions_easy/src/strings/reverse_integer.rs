// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/880/

pub struct Solution {}

impl Solution {
    pub fn reverse_integer_solution() {
        let x1 = 123;
        let x2 = -123;
        let x3 = 120;

        let r1 = Solution::reverse(x1);
        let r2 = Solution::reverse(x2);
        let r3 = Solution::reverse(x3);

        println!("{}", r1);
        println!("{}", r2);
        println!("{}", r3);
    }

    fn reverse(x: i32) -> i32 {
        let s = x.abs().to_string();
        let reversed_str: String = s.chars().rev().collect();
        let reversed_num = reversed_str.parse::<i32>().unwrap_or(0);

        if x.is_negative() {
            -reversed_num
        } else {
            reversed_num
        }
    }
}

#[allow(dead_code)]
fn main () {
    Solution::reverse_integer_solution();
}
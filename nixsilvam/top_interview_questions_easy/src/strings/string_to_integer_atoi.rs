// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/884/


pub struct Solution {}

impl Solution {
    pub fn my_atoi_solution () {
        let s1 = String::from("42");
        let s2 = String::from("-042");
        let s3 = String::from("337c0d3");
        let s4 = String::from("0-1");
        let s5 = String::from("words and 987");

        let r1 = Solution::my_atoi(s1);
        let r2 = Solution::my_atoi(s2);
        let r3 = Solution::my_atoi(s3);
        let r4 = Solution::my_atoi(s4);
        let r5 = Solution::my_atoi(s5);

        println!("{}", r1);
        println!("{}", r2);
        println!("{}", r3);
        println!("{}", r4);
        println!("{}", r5);

    }

    fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars().collect::<Vec<char>>();
        let mut sign = 1;

        if !chars.is_empty() && (chars[0] == '-' || chars[0] == '+') {
            if chars[0] == '-' {
                sign = -1;
            }
            chars.remove(0);
        }

        let mut digits = Vec::new();

        for &c in &chars {
            if c.is_ascii_digit() {
                digits.push(c);
            }
            else {
                break;
            }
        }

        if digits.is_empty() {
            return 0
        } else {
            let result = digits
                .into_iter()
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
                * sign as f64;

            if result > i32::MAX as f64 {
                return i32::MAX
            } else if result < i32::MIN as f64 {
                return i32::MIN
            } else {
                return result as i32
            }
        }
    }
}

#[allow(dead_code)]
fn main () {
    Solution::my_atoi_solution();
}
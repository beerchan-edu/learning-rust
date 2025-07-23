// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/884/

pub struct Solution {}

impl Solution {
    pub fn my_atoi_solution () {

    }

    fn my_atoi(s: String) -> i32 {
        let s = s.trim_start().chars().collect::<Vec<char>>();
        let p_sign = true;

        if s[0] != '-' {
            let p_sign = false;
            s.remove(0);
        }

        if s[0] == '0' {
            s.remove(0);
        }

        let mut v = Vec::new();

        for (i, c) in s.iter().enumerate() {
            if c.is_numeric() {
                v.push(c);
            } else {
                s.truncate(i);
            }
        }

        if v.is_empty() {
            return 0
        } else {

        }

    }
}
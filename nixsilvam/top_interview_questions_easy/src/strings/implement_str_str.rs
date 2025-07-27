// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/885/

pub struct Solution {}

impl Solution {
    pub fn str_solution() {
        let haystack1 = String::from("sadbutsad");
        let needle1 = String::from("sad");
        let haystack2 = String::from("leetcode");
        let needle2 = String::from("leeto");

        let r1 = Solution::str_str(haystack1, needle1);
        let r2 = Solution::str_str(haystack2, needle2);

        println!("{}", r1);
        println!("{}", r2);
    }

    fn str_str(haystack: String, needle: String) -> i32 {
        let v: Vec<(usize, &str)>= haystack.match_indices(&needle).collect();
        if v.is_empty() {
            return -1
        } else {
            return v[0].0 as i32
        }
    }
}

#[allow(dead_code)]
fn main () {
    Solution::str_solution();
}
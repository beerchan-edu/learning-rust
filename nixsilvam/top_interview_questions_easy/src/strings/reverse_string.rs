// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/879/

pub struct Solution {}

impl Solution {
    pub fn reverse_string_solution() {
        let mut s1: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];

        Solution::reverse_string(&mut s1);
        Solution::reverse_string(&mut s2);

        println!("{:?}", s1);
        println!("{:?}", s2);
    }

    fn reverse_string(s: &mut Vec<char>) {
        return s.reverse()
    }
}

#[allow(dead_code)]
fn main() {
    Solution::reverse_string_solution();
}
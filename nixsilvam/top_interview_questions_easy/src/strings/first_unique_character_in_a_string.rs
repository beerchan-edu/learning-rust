// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/881/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn first_uniq_char_solution () {
        let s1= String::from("leetcode");
        let s2= String::from("loveleetcode");
        let s3= String::from("aabb");

        let c1 = Solution::first_uniq_char(s1);
        let c2 = Solution::first_uniq_char(s2);
        let c3 = Solution::first_uniq_char(s3);

        println!("{}", c1);
        println!("{}", c2);
        println!("{}", c3);
    }

    fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let v: Vec<char> = s.chars().collect();

        for l in &v {
            map.entry(*l).and_modify(|count| *count += 1).or_insert(1);
        }

        for idx in 0..v.len() {
            if map.get(&v[idx]) == Some(&1) {
                return idx as i32
            }
        }

        return -1;
    }
}

#[allow(dead_code)]
fn main () {
    Solution::first_uniq_char_solution();
}

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/882/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn anagram() {
        let s1 = String::from("anagram");
        let t1 = String::from("nagaram");

        let s2 = String::from("rat");
        let t2 = String::from("car");

        let is_anagram1 = Solution::is_anagram_hashmap(s1, t1);
        let is_anagram2 = Solution::is_anagram_hashmap(s2, t2);

        println!("{}", is_anagram1);
        println!("{}", is_anagram2);
    }

    fn is_anagram_sort(s: String, t: String) -> bool {
        let mut svec: Vec<char> = s.chars().collect();
        let mut tvec: Vec<char> = t.chars().collect();
        svec.sort();
        tvec.sort();
        svec == tvec
    }

    fn is_anagram_hashmap(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts = HashMap::new();

        for ch in s.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(count) = counts.get_mut(&ch) {
                *count -= 1;
                if *count == 0 {
                    counts.remove(&ch);
                }
            } else {
                return false;
            }
        }

        counts.is_empty()
    }
}

#[allow(dead_code)]
fn main() {
    Solution::anagram();
}
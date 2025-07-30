// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/887/

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix_solution() {
        let inpt1: Vec<String> = vec![String::from("flower"),
                                String::from("flow"),
                                String::from("flight")];
        let inpt2: Vec<String> = vec![String::from("dog"),
                                String::from("racecar"),
                                String::from("car")];

        let outpt1 = Solution::longest_common_prefix(inpt1);
        let outpt2 = Solution::longest_common_prefix(inpt2);

        println!("{}", outpt1);
        println!("{}", outpt2);
    }

    fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs_len = strs.len();
        let mut prefix = String::new();

        if strs_len == 1 {
            return strs[0].clone();
        }

        let min_len = strs
            .iter()
            .map(|w| w.len())
            .min().
            unwrap_or(0);

        if min_len == 0 {
            return prefix;
        }

        for i in 0..min_len {
            let letter = strs[0].chars().nth(i).unwrap();
            for s in 1..strs.len() {
                if strs[s].chars().nth(i).unwrap() != letter {
                    return prefix;
                }
            }
            prefix.push(letter);
        }
        prefix
    }
}

#[allow(dead_code)]
fn main() {
    Solution::longest_common_prefix_solution();
}
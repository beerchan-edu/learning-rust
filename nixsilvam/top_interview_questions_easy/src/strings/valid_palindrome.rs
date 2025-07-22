// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/883/

pub struct Solution {}

impl Solution {
    pub fn palindrome() {
        let s1 = String::from("A man, a plan, a canal: Panama");
        let s2 = String::from("race a car");
        let s3 = String::from(" ");

        let p1 = Solution::is_palindrome(s1);
        let p2 = Solution::is_palindrome(s2);
        let p3 = Solution::is_palindrome(s3);

        println!("{}", p1);
        println!("{}", p2);
        println!("{}", p3);
    }
    // woaahh what a rustic beauty!
    fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        s
            .chars()
            .zip(s.chars().rev())
            .take(s.len() / 2)
            .all(|(l, r)|l == r)
    }
}

#[allow(dead_code)]
fn main() {
    Solution::palindrome();
}
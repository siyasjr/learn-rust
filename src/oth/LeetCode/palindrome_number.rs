// Problem: https://leetcode.com/problems/palindrome-number/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        s.chars().rev().collect::<String>() == s
    }
}

// Problem: https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
    }
}

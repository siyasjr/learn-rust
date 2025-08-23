// Problem: https://leetcode.com/problems/search-insert-position/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &n) in nums.iter().enumerate() {
            if n >= target {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

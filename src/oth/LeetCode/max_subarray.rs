// Problem: https://leetcode.com/problems/maximum-subarray/

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut best = nums[0];

        for &n in nums.iter().skip(1) {
            current = current.max(n + current);
            best = best.max(current);
        }
        best
    }
}

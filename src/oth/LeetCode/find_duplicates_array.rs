//Problem: Given an array of integers where 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice. Find all duplicates.

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = vec![];
        for i in 0..nums.len() {
            let idx = (nums[i].abs() - 1) as usize;
            if nums[idx] < 0 {
                res.push(idx as i32 + 1);
            } else {
                nums[idx] = -nums[idx];
            }
        }
        res
    }
}

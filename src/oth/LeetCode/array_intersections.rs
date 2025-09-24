// Problem: Return an array of their intersection, including duplicate elements.

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for &num in &nums1 { *map.entry(num).or_insert(0) += 1; }
        let mut res = vec![];
        for &num in &nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    res.push(num);
                    *count -= 1;
                }
            }
        }
        res
    }
}

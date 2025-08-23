// Problem: https://leetcode.com/problems/add-binary/

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let (mut i, mut j) = (a.len() as i32 - 1, b.len() as i32 - 1);
        let mut result = String::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;
            if i >= 0 { sum += a.chars().nth(i as usize).unwrap().to_digit(2).unwrap() as i32; i -= 1; }
            if j >= 0 { sum += b.chars().nth(j as usize).unwrap().to_digit(2).unwrap() as i32; j -= 1; }
            result.push_str(&(sum % 2).to_string());
            carry = sum / 2;
        }
        result.chars().rev().collect()
    }
}

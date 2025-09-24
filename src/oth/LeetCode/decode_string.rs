// Problem: Given an encoded string like "3[a2[c]]", decode it to "accaccacc".


impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        let mut num = 0;
        let mut res = String::new();
        
        for c in s.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap();
            } else if c == '[' {
                stack.push((res.clone(), num));
                res.clear();
                num = 0;
            } else if c == ']' {
                let (prev, count) = stack.pop().unwrap();
                res = prev + &res.repeat(count as usize);
            } else {
                res.push(c);
            }
        }
        res
    }
}

// Problem: https://leetcode.com/problems/roman-to-integer/

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let chars: Vec<char> = s.chars().collect();
        let mut result = 0;
        for i in 0..chars.len() {
            let val = map[&chars[i]];
            if i + 1 < chars.len() && val < map[&chars[i + 1]] {
                result -= val;
            } else {
                result += val;
            }
        }
        result
    }
}

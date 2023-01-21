use std::collections::HashMap;


impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        // Middle of palindrome can be odd
        // So we take longest odd string
        let mut odd = 0;
        let mut longest = 0;
        for (k, v) in map.iter() {
            if(v % 2 == 0) {
                longest += v;
            } else {
                longest += v - 1;
                odd = 1;
            }
        }
        longest + odd
    }
}
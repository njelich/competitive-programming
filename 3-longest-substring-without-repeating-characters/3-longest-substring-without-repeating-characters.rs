use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut idx = 0;
        let mut chars = HashSet::new();
        
        for c in s.chars() {
            while chars.contains(&c) {
                chars.remove(&s.chars().nth(idx).unwrap());
                idx += 1;
            }
            
            chars.insert(c);
            
            max = max.max(chars.len());
        }
        
        max as i32
    }
}
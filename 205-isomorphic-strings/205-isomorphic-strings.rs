use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut chars = HashMap::new();
        
        for i in 0..s.len() {
            let v = chars.entry(s[i]).or_insert(t[i]);
            
            if *v != t[i] {
                return false;
            }
        }

        chars.keys().collect::<HashSet<_>>().len() == chars.values().collect::<HashSet<_>>().len()
    }
}
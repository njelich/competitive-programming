use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut chars = HashMap::new();
        println!("{}", s);
        
        for (end, c) in s.char_indices() {
            let dupe = chars.entry(c).or_insert(end);
            if end != *dupe {
                max = max.max(end - start);
                if(*dupe >= start) {
                    start = *dupe + 1;
                }
                *dupe = end;
            }
        }
        
        max.max(s.len() - start) as i32
    }
}

// 0 d, a: 0;
// 1 w, 
// 2 d, idx = 2, dupe = 2
// 3 f

// 0 d, a - 0
// 1 w, b - 1, a - 0
// 2 d, idx = 2, dupe = 2
// 3 w
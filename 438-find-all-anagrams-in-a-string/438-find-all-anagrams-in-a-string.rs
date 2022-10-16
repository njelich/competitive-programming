use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s2: String, s1: String) -> Vec<i32> {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut f: HashMap<char, i32> = HashMap::with_capacity(s1.len());
        let mut l = 0;
        
        let mut res = Vec::new();
        
        for i in 0..s1.len() {
            *f.entry(s1[i]).or_default() += 1;
        }
        
        for i in 0..s2.len() {
            if(i >= s1.len()) {
                if let Some(mut e) = f.get_mut(&s2[l]) {
                    *e += 1;
                }
                l += 1;
            }
            
            if let Some(mut e) = f.get_mut(&s2[i]) {
                *e -= 1;
            }

            if(i + 1 >= s1.len()) {
                let mut sum = 0;
                for v in f.values() {
                    sum += v.abs();
                }
                if (sum == 0) {
                    res.push(l as i32);
                }
                
            }
            
        }
        
        res
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s2: String, s1: String) -> Vec<i32> {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut f_s1: HashMap<char, i32> = HashMap::with_capacity(s2.len());
        let mut f_s2: HashMap<char, i32> = HashMap::with_capacity(s2.len());
        let mut l = 0;
        
        let mut res = Vec::new();
        
        for i in 0..s2.len() {
            *f_s2.entry(s2[i]).or_default() += 1;
            if(i < s1.len()) {
                *f_s1.entry(s1[i]).or_default() += 1;
            } else {
                let c = f_s2.entry(s2[l]).or_default();
                if *c == 1 { 
                    f_s2.remove(&s2[l]);
                } else {
                    *c -= 1;
                }
                l += 1;
            }

            if(i + 1 >= s1.len() && f_s1 == f_s2) {
                res.push(l as i32);
            }
            
        }
        
        res
    }
}
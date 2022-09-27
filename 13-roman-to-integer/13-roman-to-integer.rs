use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let values: HashMap<char, i32> = HashMap::from([
            ('I', 1),   
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        
        let mut sum = 0; 
        let mut prev = 0;
        
        for c in s.chars().rev() {
            if let Some(val) = values.get(&c) {
                if *val < prev {
                    sum -= val;
                } else {
                    sum += val;
                }
                prev = *val;
            }
        }
        sum
    }
}
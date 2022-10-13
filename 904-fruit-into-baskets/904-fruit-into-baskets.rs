use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut frequency: HashMap<i32, i32> = HashMap::new();
        let mut idx = 0;
        let mut res = 0;

        for fruit in &fruits {
            let count = frequency.entry(*fruit).or_insert(0_i32);
            *count += 1;
            
            // number of baskets = 2
            while frequency.len() > 2 {
                match frequency.remove(&fruits[idx]) {
                    Some(v) if v > 1 => {
                        frequency.insert(fruits[idx], v - 1);
                    }
                    _ => {}
                }

                
                idx+=1;
            }
            
            let mut total = 0;
            for (_, nums) in &frequency {
                total += nums;
            }
            
            res = res.max(total);
        }
        
        res
    }
}
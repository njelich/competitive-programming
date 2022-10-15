use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut max = 0;
        let mut count = 0;
        let mut start = 0;
        let mut chars = HashMap::new();
        println!("{}", k);
        
        for end in 0..s.len() {
            *chars.entry(s[end]).or_insert(0) += 1;
            count = count.max(*chars.entry(s[end]).or_insert(0));
            max = max.max(end - start);
            while end - start + 1 > (k as usize) + count {
                println!("w: {} {} {}", start, end, count);
                *chars.entry(s[start]).or_insert(0) -= 1;
                start += 1;
                
            }
            
            println!("e: {} {} {} {}", start, end, count, s[end]);
        }
        
        max.max(s.len() - start) as i32
    }
}


//   unsafe {
//         let count = chars.entry(s[end]).or_insert(0);
//         *count += 1;
//         let count = count.clone();
//         while end - start > (k as usize) + count {
//             println!("while: {} {} {} {}", start, end, (k as usize), count);
//             *chars.get_mut(s[start]).unwrap() -= 1;
//             start += 1;

//         }
//     }
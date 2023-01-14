impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        
        if (s.len() == 0) {
            return true;
        }
        
        // Indexing backwards for the lolz
        let mut i: i32 = s.len() as i32 - 1;
        let mut j: i32 = t.len() as i32 - 1;
        
        while i >= 0 && j >= 0 {
            if(s[i as usize] == t[j as usize]) {
                i-=1;
            }
            j-=1;
        }
        
        i == -1
    }
}
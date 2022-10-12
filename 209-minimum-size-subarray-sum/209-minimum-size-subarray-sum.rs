use std::cmp;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut curr, mut idx, mut len) = (0, 0, usize::MAX);
        
        for i in 0..nums.len() {
            curr += nums[i];
            // if >= target, save length, reduce
            while curr >= target {
                curr -= nums[idx];
                len = cmp::min(len, i - idx + 1); 
                idx+=1;
            }
        }
        
        if(len == usize::MAX) {
            return 0;
        } else {
            len as i32
        }
        
    }
}
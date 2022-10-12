use std::cmp;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {    
        let (mut curr, mut max) = (nums[0], nums[0]); 
        
        for i in 1..nums.len() {
            // We track two maximum subarray sums
            // curr - the subarray ends on current index i
            // max - the subarray ends on any index before i (including)
            curr = cmp::max(nums[i],curr + nums[i]);
            max = cmp::max(max, curr);
        }
        
        max
    }
}
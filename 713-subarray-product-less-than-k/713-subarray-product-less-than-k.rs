impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut found = 0;
        
        let mut res;
        
        for i in 0..nums.len() {
            res = nums[i];
            if(res >= k) {
                continue;
            }
            found+=1;
            for r in (i+1)..nums.len() {
                res*=nums[r];
                if(res >= k) {
                    break;
                }
                found+=1;
            }
        }
        
        found
    }
}
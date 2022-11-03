impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        
        let mut found = 0;
        let mut res = 1;
        let mut l = 0;
        
        for r in 0..nums.len() {
            res*=nums[r];
            while(res >= k && l <= r) {
                res /= nums[l];
                l+=1;
            }
            found += r - l + 1;
        }
        
        found as i32
    }
}
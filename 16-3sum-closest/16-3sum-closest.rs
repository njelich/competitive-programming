impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {        
        nums.sort_unstable();
        
        let mut found: i32 = nums[0] + nums[1] + nums[nums.len() - 1];
        
        let (mut l, mut r, mut res) = (0, 0, 0);
        
        for i in 0..(nums.len() - 2) {
             //Some checks that make a massive memory savings in the `found` vector
            // if i > 1 && nums[i] == nums[i - 1] {
            //     continue;
            // }
            l = i + 1;
            r = nums.len() - 1;
            res = -nums[i] + target;
            while r != l {
                if((found - target).abs() > (nums[i] + nums[r] + nums[l] - target).abs()) {
                    found = nums[i] + nums[r] + nums[l];
                }
                
                // //Some checks that make a massive memory savings in the `found` vector
                // if l > i + 1 && nums[l] == nums[l - 1] {
                //     l += 1;
                //     continue;
                // }
                // if r < nums.len() - 1 && nums[r] == nums[r+1] {
                //     r -= 1;
                //     continue;
                // }
                
                if nums[i] + nums[r] + nums[l] > target {
                    r-=1;
                } else if nums[i] + nums[r] + nums[l] < target {
                    l+=1;
                } else {
                    return nums[i] + nums[r] + nums[l];
                }
            }
        }
        
        found
    }
}
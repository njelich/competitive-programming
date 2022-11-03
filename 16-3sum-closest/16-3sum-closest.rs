impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {        
        nums.sort_unstable();
        
        let mut found: i32 = nums[0] + nums[1] + nums[nums.len() - 1];
        
        let (mut l, mut r) = (0, 0);
        
        for i in 0..(nums.len() - 2) {
            l = i + 1;
            r = nums.len() - 1;
            while r != l {
                if((found - target).abs() > (nums[i] + nums[r] + nums[l] - target).abs()) {
                    found = nums[i] + nums[r] + nums[l];
                }
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
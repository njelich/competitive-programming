impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {        
        nums.sort_unstable();
        
        let mut found: i32 = nums[0] + nums[1] + nums[nums.len() - 1];
        
        let (mut l, mut r) = (0, 0);
        
        for i in 0..(nums.len() - 2) {
            l = i + 1;
            r = nums.len() - 1;
            while r != l {
                let sum = nums[i] + nums[r] + nums[l] - target;
                if((found - target).abs() > (sum).abs()) {
                    found = sum + target;
                }
                if sum > 0 {
                    r-=1;
                } else if sum < 0 {
                    l+=1;
                } else {
                    return sum + target;
                }
            }
        }
        
        found
    }
}
impl Solution {
    pub fn pivot_index(mut nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut leftsum = 0;
        for i in 0..nums.len() {
            if (2*leftsum + nums[i] == sum) {
                return i as i32;
            }
            leftsum += nums[i];
        }
        return -1
    }
}
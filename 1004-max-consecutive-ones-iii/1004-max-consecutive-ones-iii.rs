impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut zeros = 0;
        let mut start = 0;
        let mut end = 0;
        println!("{}", k);
        
        for i in nums.iter() {
            zeros += (1 - i); //if 0 add 1
            
            if(zeros > k) {
                zeros -= (1 - nums[start]);
                start += 1;
            }
            
            end += 1;
        }
        
        (end - start) as i32
    }
}
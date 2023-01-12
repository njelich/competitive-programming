impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if(nums[i] == i as i32 || nums[i] as usize >= nums.len()) {
                i+=1;
            } else {
                let n_i = nums[i] as usize;
                nums.swap(i, n_i);
            }
        }
        i = 0;
        while i < nums.len() {
            if(nums[i] != i as i32) {
                break;
            } else {
                i+=1;
            }
        }
        i as i32
    }
}
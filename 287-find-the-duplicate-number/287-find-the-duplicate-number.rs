impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let n = nums.len();
        // Cyclic sort
        while i < n {
            let n_i = (nums[i] - 1) as usize;
            if(n_i != i && n_i < n && n_i != (nums[n_i] - 1) as usize) {
                nums.swap(i, n_i);
            } else {
                i+=1;
            }
        }
        // Scan for out of place number
        for j in 0..n {
            if(nums[j] - 1 != j as i32) {
                i = nums[j] as usize;
                break;
            }
        }
        i as i32
    }
}
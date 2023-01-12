impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let n = nums.len();
        // Cyclic sort
        while i < n {
            let n_i = (nums[i] - 1) as usize;
            if(n_i != i && n_i < n && n_i != (nums[n_i] - 1) as usize && n_i >= 0) {
                nums.swap(i, n_i);
            } else {
                i+=1;
            }
        }
        // Scan for out of place numbers
        let mut ans = 0;
        for i in 0..n {
            if(nums[i] - 1 != i as i32) {
                ans = i + 1;
                break;
            }
        }
        // Handle undocumented case when there are no missing numbers
        if ans == 0 {
            ans = nums[n - 1] as usize + 1;
        }
        ans as i32
    }
}
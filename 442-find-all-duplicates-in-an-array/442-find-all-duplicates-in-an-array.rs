impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
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
        // Scan for out of place numbers
        let mut ans = Vec::new();
        for i in 0..n {
            if(nums[i] - 1 != i as i32) {
                ans.push(nums[i]);
            }
        }
        ans
    }
}
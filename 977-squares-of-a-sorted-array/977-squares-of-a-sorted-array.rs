impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![nums[0].pow(2)];
        }
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        let (mut l, mut r) = (0, nums.len() - 1);
        unsafe {
            res.set_len(r+1);
            while l != r {
                if (nums[l].abs() > nums[r]) {
                    *res.get_unchecked_mut(r-l) = nums[l].pow(2);
                    l += 1;
                } else {
                    *res.get_unchecked_mut(r-l) = nums[r].pow(2);
                    r -= 1;
                }
            }
            *res.get_unchecked_mut(0) = nums[l].pow(2);
        }
        res
    }
}
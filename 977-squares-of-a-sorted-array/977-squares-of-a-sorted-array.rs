impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![nums[0].pow(2)];
        }
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        let (mut l, mut r) = (0, nums.len() - 1);
        while l != r {
            if (nums[l].abs() > nums[r]) {
                res.push(nums[l].pow(2));
                l += 1;
            } else {
                res.push(nums[r].pow(2));
                r -= 1;
            }
        }
        res.push(nums[l].pow(2));
        res.reverse();
        res
    }
}
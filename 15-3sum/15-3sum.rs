impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut found = Vec::new();

        if nums.len() < 3 {
            return found;
        }

        nums.sort();

        let (mut l, mut r, mut res) = (0, 0, 0);
        
        for i in 0..(nums.len() - 2) {
            l = i + 1;
            r = nums.len() - 1;
            res = -nums[i];
            while r != l {
                if nums[r] + nums[l] > res {
                    r-=1;
                } else if nums[r] + nums[l] < res {
                    l+=1;
                } else {
                    found.push(vec![nums[i], nums[l], nums[r]]);
                    l+=1;
                }
            }
        }
        
        found.sort();
        found.dedup();
        found
    }
}
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut found = Vec::with_capacity(nums.len() / 3);

        if nums.len() < 3 {
            return found;
        }

        nums.sort_unstable();

        let (mut l, mut r, mut res) = (0, 0, 0);
        
        for i in 0..(nums.len() - 2) {
            //Some checks that make a massive memory savings in the `found` vector
            if i > 1 && nums[i] == nums[i - 1] {
                continue;
            }
            l = i + 1;
            r = nums.len() - 1;
            res = -nums[i];
            while r != l {
                //Some checks that make a massive memory savings in the `found` vector
                if l > i + 1 && nums[l] == nums[l - 1] {
                    l += 1;
                    continue;
                }
                if r < nums.len() - 1 && nums[r] == nums[r+1] {
                    r -= 1;
                    continue;
                }

                
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
        
        // Sorting a really small vector like this is cheaper than the extra checks
        found.sort_unstable();
        found.dedup();
        found
    }
}
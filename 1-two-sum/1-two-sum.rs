use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut numMap = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let needed = target - num;
            match numMap.get(&needed) {
                Some(id) => {
                    return vec![idx as i32, *id as i32];
                },
                None => {
                    numMap.insert(num, idx);
                }
            }
        }
        unreachable!()
    }
}
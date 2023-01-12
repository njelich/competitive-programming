impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();

        let mut res = Vec::with_capacity(intervals.len());

        let (mut left, mut right) = (intervals[0][0], intervals[0][1]);
        for i in 1..intervals.len() {
            if(intervals[i][0] <= right) {
                right = right.max(intervals[i][1]);
            } else {
                res.push(vec![left, right]);
                left = intervals[i][0];
                right = intervals[i][1];
            }
        }

        if(res.len() == 0 || res[res.len() - 1][0] != left) {
            res.push(vec![left, right]);
        }

        res
    }
}
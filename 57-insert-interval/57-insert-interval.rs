impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }

        let mut res = Vec::with_capacity(intervals.len());

        let (mut left, mut right) = (intervals[0][0], intervals[0][1]);

        let mut inserted = false;
        
        if(new_interval[0] < left) {
            left = new_interval[0];
            right = new_interval[1];
            inserted = true;
        }
        for i in 0..intervals.len() {
            if(intervals[i][0] <= right) {
                right = right.max(intervals[i][1]);
            } else {
                res.push(vec![left, right]);
                left = intervals[i][0];
                right = intervals[i][1];
            }
            if(!inserted && new_interval[0] >= intervals[i][0]) {
                if (i == intervals.len() - 1 || new_interval[0] <= intervals[i+1][0]) {
                    inserted = true;
                    if(new_interval[0] <= right) {
                        right = right.max(new_interval[1]);
                    } else {
                        res.push(vec![left, right]);
                        left = new_interval[0];
                        right = new_interval[1];
                    }
                }
            }
        }

        if(res.len() == 0 || res[res.len() - 1][0] != left) {
            res.push(vec![left, right]);
        }

        res

    }
}
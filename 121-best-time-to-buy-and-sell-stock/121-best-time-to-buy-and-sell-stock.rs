impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut low = prices[0];
        let mut high = prices[0];
        for i in prices {
            if i < low {
                low = i;
                high = i;
            } else if i > high {
                high = i;
            }
            max = max.max(high - low);
        }
        max
    }
}
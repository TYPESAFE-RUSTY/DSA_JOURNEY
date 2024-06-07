impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = i32::MIN;
        let mut i: usize = 0;
        let mut j: usize = 0;

        while j < prices.len() {
            // move window
            if prices[i] > prices[j] {
                i = j;
                continue;
            }

            // calculate profit
            profit = i32::max(profit, prices[j] - prices[i]);
            j += 1;
        }

        profit
    }
}

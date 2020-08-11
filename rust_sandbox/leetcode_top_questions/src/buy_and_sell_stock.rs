#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    /// The idea is as we iterate through the prices, we maintain the minimal price we've seen
    /// so far. For each price, we calculate the "profit" for that given day from the minimal price,
    /// and update the "max profit".
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;
        let mut min_price = prices[0];

        for &price in prices.iter().skip(1) {
            if price < min_price {
                min_price = price;
            }
            let profit = price - min_price;
            if profit > max_profit {
                max_profit = profit;
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}

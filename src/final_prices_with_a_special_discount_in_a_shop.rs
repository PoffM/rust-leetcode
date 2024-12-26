struct Solution {}

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        for i in 0..prices.len() {
            if let Some(j) = (i + 1..prices.len()).find(|&j| prices[j] <= prices[i]) {
                prices[i] -= prices[j];
            }
        }
        prices
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::final_prices(vec![8, 4, 6, 2, 3]),
        vec![4, 2, 4, 2, 3]
    );
}

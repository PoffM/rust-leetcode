struct Solution {}

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let mut result = nums;

        for n in 1..k as usize {
            let mut powers = vec![];
            for w in result.windows(2) {
                if let [left, right] = *w {
                    powers.push(if left + 1 == right { right } else { -1 })
                }
            }
            result = powers;
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
        vec![3, 4, -1, -1, -1]
    )
}

struct Solution {}

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect::<Vec<_>>();

        nums.sort_unstable();

        let mut disabled = vec![false; nums.len()];

        let mut score = 0_i64;

        for &(num, i) in nums.iter() {
            if disabled[i] {
                continue;
            }

            score += num as i64;

            if i > 0 {
                if let Some(low) = disabled.get_mut(i - 1) {
                    *low = true;
                }
            }
            if i < nums.len() - 1 {
                if let Some(high) = disabled.get_mut(i + 1) {
                    *high = true;
                }
            }
        }

        score
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
}

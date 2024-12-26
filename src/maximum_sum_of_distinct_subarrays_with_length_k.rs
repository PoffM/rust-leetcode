use std::{collections::HashMap, iter::once};

struct Solution {}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut counts = HashMap::<i32, usize>::new();
        for num in &nums[0..k as usize] {
            *counts.entry(*num).or_insert(0) += 1;
        }

        let mut sum = counts
            .iter()
            .fold(0, |acc, (num, count)| acc + (*num as i64 * *count as i64));

        let mut dupe_count: usize = counts.iter().fold(0, |acc, (_num, count)| acc + count - 1);

        let initial_sum = if counts.values().any(|count| *count > 1) {
            0
        } else {
            sum
        };

        once(initial_sum)
            .chain((1..=nums.len() - k as usize).map(|i| {
                // Decrement the left num
                {
                    let left = &nums[i - 1];

                    let count = counts.get_mut(left).unwrap();
                    *count -= 1;

                    if *count >= 1 {
                        dupe_count -= 1;
                    }

                    if *count == 0 {
                        counts.remove(left);
                    }

                    sum -= *left as i64;
                }

                // Increment the right num
                {
                    let right = &nums[i + k as usize - 1];

                    let count = counts.entry(*right).or_insert(0);
                    *count += 1;

                    if *count > 1 {
                        dupe_count += 1;
                    }

                    sum += *right as i64;
                }

                if dupe_count == 0 {
                    sum.clone()
                } else {
                    0
                }
            }))
            .max()
            .unwrap_or(0) as i64
    }
}

#[test]
fn test_maximum_subarray_sum() {
    assert_eq!(
        Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
        15
    );
}

#[test]
fn test_maximum_subarray_sum_2() {
    assert_eq!(
        Solution::maximum_subarray_sum(vec![1, 1, 1, 7, 8, 9], 3),
        24
    );
}

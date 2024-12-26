use std::{collections::HashMap, fmt::format};

struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut counts = HashMap::new();

        for col in matrix {
            let compact = col
                .iter()
                .map(if col[0] == 0 {
                    |num: &i32| if *num == 1 { true } else { false }
                } else {
                    |num: &i32| if *num == 1 { false } else { true }
                })
                .collect::<Vec<bool>>();

            *counts.entry(compact).or_insert(0) += 1;
        }

        *counts.values().max().unwrap()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
        2
    );
}

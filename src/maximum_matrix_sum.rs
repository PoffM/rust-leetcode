struct Solution {}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut odd_negatives = false;
        let mut lowest_abs = i64::MAX;
        let mut sum = 0;

        for row in matrix {
            for num in row {
                if num < 0 {
                    odd_negatives = !odd_negatives;
                }

                let abs = (num as i64).abs();
                if abs < lowest_abs {
                    lowest_abs = abs;
                }

                sum += abs;
            }
        }

        if odd_negatives {
            sum - 2 * lowest_abs
        } else {
            sum
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
        16
    );
}

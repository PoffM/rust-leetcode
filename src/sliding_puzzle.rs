use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let target: Vec<u8> = vec![1, 2, 3, 4, 5, 0];

        let moves = [
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![1, 3, 5],
            vec![2, 4],
        ];

        let mut combos = HashSet::new();
        combos.insert(
            board
                .iter()
                .flat_map(|row| row.iter().map(|num| *num as u8))
                .collect::<Vec<_>>(),
        );

        for count in 0..=20 {
            if combos.contains(&target) {
                return count;
            }

            combos = combos
                .iter()
                .flat_map(|combo| {
                    let zero_idx = combo.iter().position(|&num| num == 0).unwrap();

                    moves[zero_idx].iter().map(move |dest| {
                        let mut attempt = combo.clone();
                        attempt.swap(zero_idx, *dest);
                        attempt
                    })
                })
                .collect::<HashSet<_>>();
        }

        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
        5
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 5, 0]]),
        0
    );
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
        -1
    );
}

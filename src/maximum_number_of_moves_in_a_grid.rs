pub struct Solution {}

use std::collections::HashMap;

type MaxMovesCache = HashMap<(usize, usize), i32>;

impl Solution {
    /**
     * Uses sliding window operation.
     * Keeps a score column as it loops through grid columns.
     * Uses a constant amount of memory.
     */
    pub fn max_moves_2(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid.get(0).map(|row| row.len()).unwrap_or(0);
        let h = grid.len();

        let mut left_scores: Vec<i32> = (0..h).map(|_| 0).collect();

        let mut max_score = 0;

        // sliding window loop comparing adjacent cols
        for col_idx in 0..(w - 1) {
            let mut right_scores: Vec<i32> = (0..h).map(|_| 0).collect();

            let left_col: Vec<i32> = (0..h).map(|row_idx| grid[row_idx][col_idx]).collect();
            let right_col: Vec<i32> = (0..h).map(|row_idx| grid[row_idx][col_idx + 1]).collect();

            // Compare each right col cell to adjacent left col cells
            for (right_idx, right_val) in right_col.iter().enumerate() {
                let mut max_left_score = -1;
                for left_offset in (-1)..=1 {
                    let left_idx = right_idx as i32 + left_offset as i32;
                    if left_idx < 0 || left_idx >= h as i32 {
                        continue;
                    }
                    let left_idx = left_idx as usize;
                    let left_val = *left_col.get(left_idx).unwrap();

                    if *right_val > left_val {
                        let ls = *left_scores.get(left_idx).unwrap_or(&0);
                        if ls > max_left_score {
                            max_left_score = ls;
                        }
                    }
                }
                right_scores[right_idx] = if max_left_score >= 0 {
                    max_left_score + 1
                } else {
                    -1
                };
            }

            left_scores = right_scores;

            if left_scores.iter().max().eq(&Some(&-1)) {
                return max_score;
            }

            max_score = *left_scores.iter().max().unwrap_or(&0).max(&0);
        }

        max_score
    }

    /**
     * Uses recursion to look down every possible path.
     * Uses a cache ("dynamic programming") to avoid going down the same path multiple times.
     * Avoids checking dead cells.
     */
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let h = grid.len();

        let mut cache: MaxMovesCache = HashMap::new();

        // loop from top to bottom
        (0..h)
            .map(|row_num| Solution::max_moves_internal(&grid, &(row_num, 0), &mut cache))
            .max()
            .unwrap_or(0)
    }

    fn max_moves_internal(
        grid: &Vec<Vec<i32>>,
        starting_point: &(usize, usize),
        cache: &mut MaxMovesCache,
    ) -> i32 {
        if let Some(cached_solution) = cache.get(starting_point) {
            return *cached_solution;
        }

        let (row_idx, col_idx) = starting_point;
        let h = grid.len();

        if *col_idx == grid.get(0).map(|r| r.len() - 1).unwrap_or(0) {
            return 0;
        }

        // check valid next cells
        let next_candidates = {
            let cell_val = grid[*row_idx][*col_idx];
            let mut results: Vec<(usize, usize)> = vec![];
            for vertical_step in -1i32..=1 {
                let next_row_idx = (*row_idx as i32) + vertical_step;
                if next_row_idx < 0 || next_row_idx >= (h as i32) {
                    continue;
                }
                // filter to higher nums
                if grid[next_row_idx as usize][col_idx + 1] > cell_val {
                    results.push((next_row_idx as usize, col_idx + 1));
                }
            }
            results
        };

        // recursively call for the next num (cache result)
        let max_moves = next_candidates
            .iter()
            .map(|next_start| 1 + Solution::max_moves_internal(&grid, next_start, cache))
            .max()
            .unwrap_or(0);

        cache.insert(*starting_point, max_moves);

        max_moves
    }
}

#[test]
fn test1() {
    let grid = vec![
        vec![2, 4, 3, 5],
        vec![5, 4, 9, 3],
        vec![3, 4, 2, 11],
        vec![10, 9, 13, 15],
    ];
    assert_eq!(Solution::max_moves(grid), 3);
    let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
    assert_eq!(Solution::max_moves(grid), 0);
    let grid = vec![
        vec![187, 167, 209, 251, 152, 236, 263, 128, 135],
        vec![267, 249, 251, 285, 73, 204, 70, 207, 74],
        vec![189, 159, 235, 66, 84, 89, 153, 111, 189],
        vec![120, 81, 210, 7, 2, 231, 92, 128, 218],
        vec![193, 131, 244, 293, 284, 175, 226, 205, 245],
    ];
    assert_eq!(Solution::max_moves(grid), 3);
}

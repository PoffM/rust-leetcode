use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid.len();
        let h = grid[0].len();
        let mut costs = vec![vec![i32::MAX; h]; w];

        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut q = BinaryHeap::from([Reverse((0, 0, 0))]);

        while let Some(Reverse((cost, x, y))) = q.pop() {
            if x == w - 1 && y == h - 1 {
                return cost;
            }

            for (dx, dy) in dirs {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let nc = cost + grid[nx][ny];
                    if nc < costs[nx][ny] {
                        costs[nx][ny] = nc;
                        q.push(Reverse((nc, nx, ny)));
                    }
                }
            }
        }

        i32::MAX
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
        2
    )
}

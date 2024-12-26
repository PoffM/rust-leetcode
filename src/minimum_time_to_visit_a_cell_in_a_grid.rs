use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid.len();
        let h = grid[0].len();

        let mut q = BinaryHeap::from([Reverse((0, 0, 0))]);

        let mut visited = vec![vec![false; h]; w];
        visited[0][0] = true;

        // The case where you can't move from the start point.
        if grid[1][0] > 1 && grid[0][1] > 1 {
            return -1;
        }

        while let Some(Reverse((t, x, y))) = q.pop() {
            if x == w - 1 && y == h - 1 {
                return t;
            }

            let mut visit = |nx: usize, ny: usize, t: i32| {
                if visited[nx][ny] {
                    return;
                }

                let nv = grid[nx][ny];
                let mut nt = t + 1;

                // If you can't get to this node, walk back and forth until your time is high enough.
                // i.e. increase by the smallest even number that gets you to the right time.
                if nt < nv {
                    nt += (nv - nt + 1) & !1;
                }

                q.push(Reverse((nt, nx, ny)));

                visited[nx][ny] = true;
            };

            if x > 0 {
                visit(x - 1, y, t);
            }
            if x < w - 1 {
                visit(x + 1, y, t);
            }
            if y > 0 {
                visit(x, y - 1, t);
            }
            if y < h - 1 {
                visit(x, y + 1, t);
            }
        }

        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
        7
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
        -1
    );
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::minimum_time(vec![
            vec![0, 1, 1000, 1000],
            vec![0, 1000, 1000, 1000],
            vec![1000, 1000, 1000, 1000]
        ]),
        1003
    )
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
        7
    )
}

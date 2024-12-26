use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn count_unguarded(h: i32, w: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut contents = HashSet::new();
        for item in guards.iter().chain(walls.iter()) {
            contents.insert((item[0], item[1]));
        }

        let mut guarded = HashSet::new();

        for g in guards.iter() {
            if let [y0, x0] = **g {
                for y1 in (0..y0).rev() {
                    if contents.contains(&(y1, x0)) {
                        break;
                    } else {
                        guarded.insert((y1, x0));
                    }
                }
                for y1 in y0 + 1..h {
                    if contents.contains(&(y1, x0)) {
                        break;
                    } else {
                        guarded.insert((y1, x0));
                    }
                }
                for x1 in (0..x0).rev() {
                    if contents.contains(&(y0, x1)) {
                        break;
                    } else {
                        guarded.insert((y0, x1));
                    }
                }
                for x1 in x0 + 1..w {
                    if contents.contains(&(y0, x1)) {
                        break;
                    } else {
                        guarded.insert((y0, x1));
                    }
                }
            }
        }

        (w * h) - guards.len() as i32 - walls.len() as i32 - guarded.len() as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]]
        ),
        7
    );
}

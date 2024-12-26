use std::collections::HashMap;

struct Solution {}

enum Item {
    Stone,
    Obstacle
}
use Item::*;

impl Solution {
    pub fn rotate_the_box(mut boxx: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut items = HashMap::new();
        let mut stone_locs = vec![];

        for (row_i, row) in boxx.iter().enumerate() {
            for (col_i, char) in row.iter().enumerate().rev() {
                match *char {
                    '*' => {
                        items.insert((row_i, col_i), Obstacle);
                    }
                    '#' => {
                        stone_locs.push((row_i, col_i));
                    }
                    _ => {}
                }
            }
        }

        let m = boxx.len();
        let n = boxx[0].len();

        boxx.clear();

        for (row_i, mut col_i) in stone_locs.iter() {
            loop {
                if items.contains_key(&(*row_i, col_i + 1)) || col_i + 1 >= n {
                    items.insert((*row_i, col_i), Stone);
                    break;
                }
                col_i += 1;
            }
        }

        for col_i in 0..n {
            let mut row: Vec<char> = vec![];
            for row_i in (0..m).rev() {
                let item = match items.get(&(row_i, col_i)) {
                    None => '.',
                    Some(item) => {
                        match item {
                            Stone => '#',
                            Obstacle => '*'
                        }
                    }
                };
                row.push(item);
            }
            boxx.push(row);
        }

        boxx
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.'],]),
        vec![
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ]
    );
}

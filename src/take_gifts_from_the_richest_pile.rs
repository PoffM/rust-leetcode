use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts);

        for _ in 0..k {
            if let Some(pile_size) = heap.pop() {
                let new_size = (pile_size as f64).sqrt().floor() as i32;
                heap.push(new_size);
            } else {
                break;
            }
        }

        heap.iter().fold(0_i64, |acc, &num| acc + num as i64)
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
}

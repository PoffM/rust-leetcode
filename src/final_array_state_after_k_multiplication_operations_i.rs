use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, mut k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::from_iter(
            nums.iter_mut()
                .enumerate()
                .map(|(i, num)| Reverse((num, i))),
        );

        while k > 0 {
            if let Some(Reverse((num, i))) = heap.pop() {
                *num *= multiplier;
                heap.push(Reverse((num, i)))
            }
            k -= 1;
        }

        nums
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
        vec![8, 4, 6, 5, 6]
    );
}

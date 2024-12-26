use std::ops::Range;


struct Solution {}

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut low = 1;
        let mut high = *quantities.iter().max().unwrap();

        let mut result = 0;

        while low <= high {
            let chunk_size = low + (high - low) / 2;
            let stores_needed = quantities
                .iter()
                .map(|&quantity| (quantity as f64 / chunk_size as f64).ceil() as i32)
                .sum::<i32>();

            if stores_needed <= n {
                result = chunk_size;
                high = chunk_size - 1;
            } else {
                low = chunk_size + 1;
            }
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5)
}

#[test]
fn test_2() {
    assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3)
}

#[test]
fn test_3() {
    assert_eq!(Solution::minimized_maximum(3, vec![2, 10, 6]), 10)
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::minimized_maximum(22, vec![25, 11, 29, 6, 24, 4, 29, 18, 6, 13, 25, 13]),
        13
    )
}

#[test]
fn test_5() {
    assert_eq!(
        Solution::minimized_maximum(15, vec![16, 24, 18, 26, 18, 28, 11, 8, 22, 26, 21, 23]),
        24
    )
}

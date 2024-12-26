struct Solution {}

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut col_counts = [0; 32];

        for num in candidates.iter() {
            for i in 0..32 {
                let bit = (num & (1 << i)) != 0;
                if bit {
                    col_counts[i] += 1;
                }
            }
        }

        *col_counts.iter().max().unwrap_or(&0)
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
        4
    );
}

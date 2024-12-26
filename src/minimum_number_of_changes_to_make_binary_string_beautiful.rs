struct Solution {}

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut chars = s.chars();
        let mut count = 0;
        while let Some(left) = chars.next() {
            if let Some(right) = chars.next() {
              if left != right {
                count += 1;
              }
            }
        }
        count
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::min_changes("1001".to_owned()), 2);
}

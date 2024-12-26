struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        for idx in 0..goal.len() {
            let goal_right = goal.chars().skip(idx);
            let goal_left = goal.chars().take(idx);

            let rearranged = goal_right.chain(goal_left);

            if s.chars().eq(rearranged) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::rotate_string("abcde".to_owned(), "cdeab".to_owned()),
        true
    );
    assert_eq!(
        Solution::rotate_string("abcde".to_owned(), "abced".to_owned()),
        false
    );
}

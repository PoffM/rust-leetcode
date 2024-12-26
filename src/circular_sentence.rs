struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words: Vec<&str> = sentence.split(" ").collect();

        for window in words.windows(2) {
            let left = window[0];
            let right = window[1];
            if left.chars().last() != right.chars().next() {
                return false;
            }
        }

        if words.last().unwrap().chars().last() != words.first().unwrap().chars().next() {
            return false;
        }

        true
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::is_circular_sentence("leetcode exercises sound delightful".to_owned()),
        true
    )
}

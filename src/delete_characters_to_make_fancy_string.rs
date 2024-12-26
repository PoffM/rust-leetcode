struct Solution {}

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut new_str = String::new();

        let mut tracked_char: char = 'a';
        let mut same_count = 0;
        for char in s.chars() {
            if char == tracked_char {
                same_count += 1;
            } else {
                tracked_char = char;
                same_count = 1;
            }

            if same_count < 3 {
                new_str.push(char);
            }
        }

        new_str
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::make_fancy_string("leeetcode".to_owned()),
        "leetcode"
    );
}

struct Solution {}

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut comp = String::new();

        let mut count = 0;
        let mut current_char = word.chars().nth(0).unwrap();

        for char in word.chars() {
            if char != current_char || count == 9 {
              comp.push_str(&format!("{}{}", count, current_char));
              count = 1;
              current_char = char;
            } else {
              count += 1;
            }
        }
        comp.push_str(&format!("{}{}", count, current_char));
        comp
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::compressed_string("aaabcdeffffffffffffffff".to_owned()),
        "3a1b1c1d1e9f7f".to_owned()
    )
}

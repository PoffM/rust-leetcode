struct Solution {}

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split_whitespace().enumerate() {
            if word.starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }
        -1
    }
}

#[test]
fn test_1() {
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
    }
}

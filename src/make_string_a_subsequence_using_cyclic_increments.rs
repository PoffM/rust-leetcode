struct Solution {}

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        Self::can_make_subsequence_str(&str1, &str2)
    }

    pub fn can_make_subsequence_str(str1: &str, str2: &str) -> bool {
        if str2.is_empty() {
            return true;
        }
        if str1.is_empty() {
            return false;
        }

        let left_base = str1.chars().nth(0).unwrap();
        let left_inc = match left_base {
            'z' => 'a',
            char => (char as u8 + 1) as char
        };
        let right = str2.chars().nth(0).unwrap();

        for i in 0..str1.len() {
            if left_base == right || left_inc == right {
                return Self::can_make_subsequence_str(&str1[i + 1..], &str2[1..]);
            } else {
                return Self::can_make_subsequence_str(&str1[i + 1..], &str2);
            }
        }

        false
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::can_make_subsequence("abc".to_string(), "ad".to_string()),
        true
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::can_make_subsequence("zc".to_string(), "ad".to_string()),
        true
    )
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::can_make_subsequence("zc".to_string(), "ad".to_string()),
        true
    )
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::can_make_subsequence("dm".to_string(), "e".to_string()),
        true
    )
}

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut spans = (0..s.len()).map(|i| (&s[i..=i], i, i)).collect::<Vec<_>>();

        let mut longest = 0;

        loop {
            let mut counts = HashMap::new();
            for (substr, start, end) in spans.iter() {
                *counts.entry(*substr).or_insert_with(|| 0) += 1;
            }

            counts.retain(|_span, v| *v >= 3);

            spans.retain(|(substr, _, end)| counts.contains_key(substr) && *end < s.len() - 1);

            if spans.is_empty() {
                if longest == 0 {
                    return -1;
                }
                return longest;
            } else {
                longest += 1;
            }

            for (substr, start, end) in spans.iter_mut() {
                *end += 1;
                *substr = &s[*start..=*end];
            }

            spans.retain(|(substr, _, _)| substr.iter().all(|&char| char == substr[0]));
        }

        longest
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
}

#[test]
fn test_2() {
    assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::maximum_length("abcccccdddd".to_string()), 3);
}

#[test]
fn test_5() {
    assert_eq!(
        Solution::maximum_length("cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde".to_string()),
        2
    );
}

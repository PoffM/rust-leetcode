struct Solution {}

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }

        let s = s.as_bytes();

        let mut counts = [0; 3];
        for char in s.iter() {
            counts[(char - b'a') as usize] += 1;
        }

        if counts.iter().any(|count| *count < k as usize) {
            return -1;
        }

        let is_valid = |counts: &[usize]| counts.iter().all(|count| *count >= k as usize);

        let mut left = 0;
        let mut right = 0;

        counts[(s[0] - b'a') as usize] -= 1;
        let mut max_size = if is_valid(&counts) { 1 } else { 0 };

        while right < s.len() - 1 {
            if is_valid(&counts) {
                right += 1;
                counts[(s[right] - b'a') as usize] -= 1;
            } else if left < right {
                counts[(s[left] - b'a') as usize] += 1;
                left += 1;
            } else {
                counts[(s[left] - b'a') as usize] += 1;
                left += 1;
                right += 1;
                counts[(s[right] - b'a') as usize] -= 1;
            }

            if is_valid(&counts) {
                let size = right - left + 1;
                if size > max_size {
                    max_size = size;
                }
            }
        }

        s.len() as i32 - max_size as i32
    }
}

#[test]
fn test_take_characters() {
    assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
}

#[test]
fn test_take_characters_2() {
    assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
}

#[test]
fn test_take_characters_3() {
    assert_eq!(Solution::take_characters("a".to_string(), 0), 0);
}

#[test]
fn test_take_characters_4() {
    assert_eq!(Solution::take_characters("abc".to_string(), 1), 3);
}

#[test]
fn test_take_characters_5() {
    assert_eq!(Solution::take_characters("acba".to_string(), 1), 3);
}

#[test]
fn test_take_characters_6() {
    assert_eq!(Solution::take_characters("bcca".to_string(), 1), 3);
}

#[test]
fn test_take_characters_7() {
    assert_eq!(Solution::take_characters("caccbbba".to_string(), 1), 3);
}

#[test]
fn test_take_characters_8() {
    assert_eq!(Solution::take_characters("aabbccca".to_string(), 2), 6);
}

struct Solution {}

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut result = "".to_string();

        let mut offset = 0;
        for space_i in spaces {
            for char in s[offset..space_i as usize].chars() {
                result.push(char)
            }
            result.push(' ');
            offset = space_i as usize;
        }
        for char in s[offset..].chars() {
            result.push(char)
        }

        result
    }

    pub fn add_spaces_using_bytes(s: String, spaces: Vec<i32>) -> String {
        let mut result = vec![0u8; s.len() + spaces.len()];
        let s = s.as_bytes();

        let mut input_i = 0;
        let mut result_i = 0;

        for space_i in spaces {
            for i in input_i..space_i as usize {
                result[result_i] = s[i];
                result_i += 1;
            }
            result[result_i] = b' ';
            result_i += 1;
            input_i = space_i as usize;
        }

        for i in input_i..s.len() as usize {
            result[result_i] = s[i];
            result_i += 1;
        }

        String::from_utf8(result).unwrap()
    }
}

#[test]
fn test_with_strs() {
    assert_eq!(
        Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
        "Leetcode Helps Me Learn".to_string()
    );
}

#[test]
fn test_with_bytes() {
    assert_eq!(
        Solution::add_spaces_using_bytes("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
        "Leetcode Helps Me Learn".to_string()
    );
}

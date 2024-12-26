struct Solution {}

impl Solution {
    pub fn repeat_limited_string(mut s: String, repeat_limit: i32) -> String {
        let mut counts = [0; 26];
        for char in s.chars() {
            counts[(char as u8 - b'a') as usize] += 1;
        }
        let mut counts = counts
            .into_iter()
            .enumerate()
            .filter(|&(_i, count)| count > 0)
            .map(|(i, count)| (i as u8 + b'a', count))
            .collect::<Vec<_>>();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        let mut result = vec![counts[0].0];
        counts[0].1 -= 1;

        let mut repeat = 1;
        'outer: loop {
            'char_search: for (char, count) in counts.iter_mut() {
                if *count == 0 {
                    continue 'char_search;
                }
                if repeat >= repeat_limit && char == result.last().unwrap() {
                    repeat = 0;
                    continue 'char_search;
                }
                if char != result.last().unwrap() {
                    repeat = 0;
                }
                result.push(*char);
                *count -= 1;

                repeat += 1;
                continue 'outer;
            }
            break;
        }

        s = String::from_utf8(result).unwrap();
        s
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::repeat_limited_string("cczazcc".to_string(), 3),
        "zzcccac".to_string()
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::repeat_limited_string("aababab".to_string(), 2),
        "bbabaa".to_string()
    );
}

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut spans = vec![];
        {
            let mut span_start = 0;
            let mut last_parity = nums[0] % 2 == 0;

            for (i, num) in nums.iter().enumerate() {
                let parity = num % 2 != 0;

                if parity == last_parity {
                    spans.push((span_start as i32, i as i32 - 1));
                    span_start = i;
                }

                last_parity = parity;
            }

            spans.push((span_start as i32, nums.len() as i32 - 1));
        }

        queries
            .iter()
            .map(|q| {
                if let [qs, qe] = q[..] {
                    if qs == qe {
                        return true;
                    }

                    return spans
                        .binary_search_by(|&(ss, se)| {
                            if ss > qs {
                                return Ordering::Greater;
                            }
                            if se < qe {
                                return Ordering::Less;
                            }
                            Ordering::Equal
                        })
                        .is_ok();
                }
                false
            })
            .collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
        vec![false]
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::is_array_special(vec![1, 8], vec![vec![1, 1]]),
        vec![true]
    )
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
        vec![false, true]
    )
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::is_array_special(vec![2, 3, 2], vec![vec![0, 1]]),
        vec![true]
    )
}

#[test]
fn test_5() {
    assert_eq!(
        Solution::is_array_special(vec![3, 6, 2, 1], vec![vec![0, 1]]),
        vec![true]
    )
}

struct Solution {}

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let len = arr.len();

        let mut prefix_end = 0;
        for w in arr.windows(2) {
            if let [a, b] = *w {
                if a <= b {
                    prefix_end += 1;
                } else {
                    break;
                }
            }
        }

        if prefix_end == len - 1 {
            return 0;
        }

        let mut suffix_start = len - 1;
        for w in arr.windows(2).rev() {
            if let [a, b] = *w {
                if a <= b {
                    suffix_start -= 1;
                } else {
                    break;
                }
            }
        }

        let mut result = (len - prefix_end - 1).min(suffix_start);

        let mut i = 0;
        let mut j = suffix_start;

        while i <= prefix_end && j < len {
            if arr[i] <= arr[j] {
                result = result.min(j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }

        result as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
        3
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
        4
    )
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0)
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![2, 2, 2, 1, 1, 1]),
        3
    )
}

#[test]
fn test_5() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 0, 7, 8, 9]),
        2
    )
}

#[test]
fn test_6() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![13, 0, 14, 7, 18, 18, 18, 16, 8, 15, 20]),
        8
    )
}

#[test]
fn test_7() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1,2,3,3,10,1,3,3,5]),
        2
    )
}

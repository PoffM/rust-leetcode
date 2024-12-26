struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut visited = 0_u16;
        let mut result = 0;
        let mut nums_needed = 0_u8;
        for (range_max, num) in (0..).zip(arr) {
            nums_needed += 1;
            visited |= 1u16 << num;
            if (visited & (1u16 << range_max)) != 0 {
                nums_needed -= 1;
            }
            if num < range_max {
                nums_needed -= 1;
            }
            if nums_needed == 0 {
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
}

#[test]
fn test_2() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![2, 0, 1]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 2, 0, 3]), 2);
}

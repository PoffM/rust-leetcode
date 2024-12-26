use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut set = BTreeSet::<(i32, usize)>::new();
        let mut result = 0;

        let is_continuous = |set: &mut BTreeSet<(i32, usize)>| -> bool {
            if let (Some((min, _)), Some((max, _))) = (set.first(), set.last()) {
                return (max - min).abs() <= 2;
            }
            true
        };

        let mut right = 0;
        for left in 0..nums.len() {
            while right < nums.len() && is_continuous(&mut set) {
                set.insert((nums[right], right));
                right += 1;
            }

            if is_continuous(&mut set) {
                result += (right - left) as i64;
            } else {
                result += (right - left - 1) as i64;
            }

            set.remove(&(nums[left], left));
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4]), 8);
}

#[test]
fn test_2() {
    assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3]), 6);
}

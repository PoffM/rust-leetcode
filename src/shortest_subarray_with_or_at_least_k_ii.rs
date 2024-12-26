struct Solution {}

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        // Frequencies of each bit position
        let mut freq = [0; 32];

        // Increment/decrement the count at the given bit position
        let update = |freq: &mut [i32], position: i32, diff: i32| {
            for i in 0..32 {
                let mask = 1 << i;
                if (mask & position) != 0 {
                    freq[i] += diff;
                }
            }
        };

        // Check if the total 'or' aggregate is >= k
        let check = |freq: &[i32]| {
            let mut r = 0;
            for i in 0..32 {
                if freq[i] > 0 {
                    r |= 1 << i;
                }
            }
            r >= k
        };

        let mut result = i32::MAX;
        let mut j = 0;

        for i in 0..nums.len() {
            update(&mut freq, nums[i], 1);

            while check(&freq) && j <= i {
                result = result.min((i - j + 1) as i32);
                update(&mut freq, nums[j], -1);
                j += 1;
            }
        }

        if result != i32::MAX {
            return result;
        }

        -1
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1)
}

#[test]
fn test_2() {
    assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3)
}

#[test]
fn test_3() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1)
}

#[test]
fn test_4() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 32, 21], 55), 3)
}
#[test]
fn test_5() {
    assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 9, 12], 21), -1)
}

struct Solution {}

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let len = nums.len();

        let mut xor = nums.iter().fold(0, |acc, &x| acc ^ x);

        let mask = (1 << maximum_bit) - 1;

        let mut result = Vec::with_capacity(len);

        for i in (0..len).rev() {
            result.push(!xor & mask);
            xor ^= nums[i];
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::get_maximum_xor(vec![0, 1, 1, 3], 2),
        vec![0, 3, 2, 3]
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::get_maximum_xor(vec![2, 3, 4, 7], 3),
        vec![5, 2, 6, 5]
    )
}

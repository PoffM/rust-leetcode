struct Solution {}

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut ones = nums[0].count_ones();
        let mut spans = vec![];

        for num in nums {
            // on new ones count, start new chunk
            if ones != num.count_ones() {
                spans.push((min, max));
                min = i32::MAX;
                max = i32::MIN;
                ones = num.count_ones();
            }

            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }
        // on last element, end chunk
        spans.push((min, max));

        for ((_, left), (right, _)) in spans.iter().zip(spans.iter().skip(1)) {
            if left > right {
                return false;
            }
        }

        true
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true);
    assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false);
}

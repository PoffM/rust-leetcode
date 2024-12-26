struct Solution {}

impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut points = nums
            .iter()
            .flat_map(|&num| [(true, num - k), (false, num + k)])
            .collect::<Vec<(bool, i32)>>();
        points.sort_unstable_by(|&(is_start_a, num_a), &(is_start_b, num_b)| {
            num_a.cmp(&num_b).then_with(|| is_start_b.cmp(&is_start_a))
        });

        let mut max = 1;
        let mut score = 0;

        for (is_start, _) in points {
            score += match is_start {
                true => 1,
                false => -1,
            };
            if score > max {
                max = score;
            }
        }

        max
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
}

#[test]
fn test_3() {
    assert_eq!(Solution::maximum_beauty(vec![13, 46, 71], 29), 3);
}

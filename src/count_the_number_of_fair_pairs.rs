struct Solution {}

impl Solution {
    // First bad attempt
    // pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    //     nums.sort_unstable();

    //     let mut counted_nums = vec![(nums[0], 0)];
    //     for num in nums.iter() {
    //         let last_num = counted_nums.last_mut().unwrap();
    //         if *num == last_num.0 {
    //             last_num.1 += 1;
    //         } else {
    //             counted_nums.push((*num, 1));
    //         }
    //     }

    //     let mut count = 0;

    //     for base in counted_nums.iter() {
    //         let add_start_idx =
    //             counted_nums.partition_point(|add| add.0 < base.0 || base.0 + add.0 < lower);

    //         let add_end_idx = counted_nums.partition_point(|add| base.0 + add.0 <= upper);

    //         if add_end_idx == 0 {
    //             continue;
    //         }

    //         for add_index in add_start_idx..add_end_idx {
    //             let add = counted_nums[add_index];
    //             if base.0 == add.0 {
    //                 count += (0..add.1).sum::<i32>();
    //             } else {
    //                 count += base.1 * add.1;
    //             }
    //         }
    //     }

    //     count as i64
    // }

    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        nums.iter()
            .enumerate()
            .skip(1)
            .fold(0, |acc, (base_idx, new)| {
                let previous_nums = &nums[0..base_idx];
                let left_idx = previous_nums.partition_point(|old| new + old < lower);
                let right_idx = previous_nums.partition_point(|old| new + old <= upper);

                acc + right_idx - left_idx
            }) as i64
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5, 100, 200], 3, 6),
        6
    )
}
#[test]
fn test_2() {
    assert_eq!(Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], 0, 0), 15)
}

#[test]
fn test_3() {
    assert_eq!(Solution::count_fair_pairs(vec![5, 7, 5, 7, 5], 12, 12), 6)
}

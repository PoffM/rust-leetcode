use std::collections::BinaryHeap;

struct Solution {}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Section {
    max_size: i32,
    num: i32,
    dividers: i32,
}

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, mut max_operations: i32) -> i32 {
        let total = nums.iter().fold(0, |acc, &num| acc + num as i64);
        let proportions = nums.iter().map(|&num| num as f64 / total as f64);

        let mut heap = nums
            .iter()
            .zip(proportions)
            .map(|(&num, proportion)| {
                let dividers = (proportion * max_operations as f64) as i32 + 1;
                let mut max_size = num / dividers;

                if num % dividers != 0 {
                    max_size += 1;
                }

                Section {
                    max_size,
                    num,
                    dividers,
                }
            })
            .collect::<BinaryHeap<_>>();

        let used_ops = heap
            .iter()
            .fold(0, |acc, section| acc + section.dividers - 1);
        max_operations -= used_ops;

        while max_operations > 0 {
            if let Some(mut section) = heap.pop() {
                section.dividers += 1;
                section.max_size = section.num / section.dividers;
                if section.num % section.dividers != 0 {
                    section.max_size += 1;
                }
                heap.push(section);
            }

            max_operations -= 1;
        }

        heap.pop().unwrap().max_size
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::minimum_size(vec![9], 2), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
}

#[test]
fn test_3() {
    assert_eq!(Solution::minimum_size(vec![7, 17], 2), 7);
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::minimum_size(
            vec![
                431, 922, 158, 60, 192, 14, 788, 146, 788, 775, 772, 792, 68, 143, 376, 375, 877,
                516, 595, 82, 56, 704, 160, 403, 713, 504, 67, 332, 26,
            ],
            80
        ),
        129
    );
}

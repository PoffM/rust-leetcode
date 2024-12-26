use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        #[derive(PartialEq, PartialOrd)]
        struct ClassData<'a> {
            val: f64,
            class: &'a mut Vec<i32>,
        }

        impl Ord for ClassData<'_> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.val.partial_cmp(&other.val).unwrap()
            }
        }

        impl Eq for ClassData<'_> {}

        let mut heap = BinaryHeap::from_iter(
            classes
                .iter_mut()
                .filter(|arr| arr[0] < arr[1]) // filter out completely passed classes
                .map(|class| {
                    let current = class[0] as f64 / class[1] as f64;
                    let next = (class[0] as f64 + 1.0) / (class[1] as f64 + 1.0);
                    let val = next - current;
                    ClassData { val, class }
                }),
        );

        if !heap.is_empty() {
            while extra_students > 0 {
                if let Some(ClassData { class, val: _ }) = heap.pop() {
                    class[0] += 1;
                    class[1] += 1;

                    let current = class[0] as f64 / class[1] as f64;
                    let next = (class[0] as f64 + 1.0) / (class[1] as f64 + 1.0);
                    let val = next - current;

                    heap.push(ClassData { class, val });
                    extra_students -= 1;
                }
            }
        }

        classes
            .iter()
            .map(|class| class[0] as f64 / class[1] as f64)
            .sum::<f64>()
            / classes.len() as f64
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
        0.78333
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
        0.53485
    );
}

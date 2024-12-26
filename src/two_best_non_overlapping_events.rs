struct Solution {}

impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        // Sort by end time:
        events.sort_unstable_by_key(|event| event[1]);

        let mut cmax = vec![0; events.len() + 1];

        events
            .iter()
            .enumerate()
            .map(|(i, event)| {
                cmax[i + 1] = cmax[i].max(event[2]);
                let j = events.partition_point(|other| other[1] < event[0]);
                event[2] + cmax[j]
            })
            .max()
            .unwrap()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3],]),
        4
    );
}

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = HashMap::new();
        let mut in_out_deg = HashMap::new();

        for pair in pairs.iter() {
            graph.entry(pair[0]).or_insert_with(|| vec![]).push(pair[1]);
            *in_out_deg.entry(pair[0]).or_insert(0) += 1;
            *in_out_deg.entry(pair[1]).or_insert(0) -= 1;
        }

        let start = in_out_deg
            .keys()
            .find(|key| in_out_deg.get(key).unwrap() == &1)
            .unwrap_or(&pairs[0][0]);

        let mut path = Vec::with_capacity(pairs.len());
        let mut stack = vec![*start];

        while !stack.is_empty() {
            if let Some(neighbors) = graph.get_mut(&stack.last().unwrap()) {
                if neighbors.is_empty() {
                    path.push(stack.pop().unwrap());
                } else {
                    let next_node = neighbors.pop().unwrap();
                    stack.push(next_node);
                }
            } else {
                path.push(stack.pop().unwrap());
            }
        }

        path.windows(2)
            .rev()
            .map(|w| vec![w[1], w[0]])
            .collect::<Vec<_>>()
    }
}

#[test]
fn test_0() {
    assert_eq!(
        Solution::valid_arrangement(vec![vec![1, 0], vec![0, 0], vec![1, 0], vec![0, 0],]),
        vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1],]
    );
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4],]),
        vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1],]
    );
}

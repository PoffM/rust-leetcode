struct Solution {}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut forwards = (0..(n - 1)).map(|i| vec![i + 1]).collect::<Vec<_>>();

        let mut scores = Vec::with_capacity(queries.len());

        for query in queries {
            let src = query[0] as usize;
            let dest = query[1] as usize;
            forwards[src].push(dest);

            let mut visited = vec![false; n];

            let mut nodes = vec![0];
            let mut score = 0;
            while !nodes.contains(&(n - 1)) {
                let mut new_nodes = vec![];
                for node in nodes {
                    for &conn in forwards[node].iter() {
                        if !visited[conn] {
                            new_nodes.push(conn);
                            visited[conn] = true;
                        }
                    }
                }
                nodes = new_nodes;
                score += 1;
            }
            scores.push(score);
        }

        scores
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
        vec![3, 2, 1]
    )
}

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        mut values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut adj_map = HashMap::new();
        for edge in edges.iter() {
            adj_map
                .entry(edge[0] as usize)
                .or_insert_with(|| vec![])
                .push(edge[1] as usize);
            adj_map
                .entry(edge[1] as usize)
                .or_insert_with(|| vec![])
                .push(edge[0] as usize);
        }

        let mut visited = vec![false; values.len()];
        let mut result = 0;

        Self::dfs(&adj_map, &mut visited, &mut values, &k, 0, &mut result);
        result
    }

    fn dfs(
        adj_map: &HashMap<usize, Vec<usize>>,
        visited: &mut Vec<bool>,
        values: &mut Vec<i32>,
        k: &i32,
        u: usize,
        result: &mut i32,
    ) -> i32 {
        visited[u] = true;
        if !adj_map.contains_key(&u) {
            *result += 1;
            return 0;
        }
        for v in adj_map[&u].iter() {
            if !visited[*v] {
                values[u] += Self::dfs(adj_map, visited, values, k, *v, result) % k;
            }
        }
        if values[u] % k == 0 {
            *result += 1;
            return 0;
        }
        values[u]
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::max_k_divisible_components(
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6
        ),
        2
    );
}

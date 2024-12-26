struct Solution {}

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, mut queries: Vec<i32>) -> Vec<i32> {
        // Sort by ascending price then ascending beauty
        items.sort_unstable();
        // Filter down to the best deal per price
        items.dedup_by(|a, b| a[1] < b[1]);

        // Binary search for max beauty for each query price
        for query in queries.iter_mut() {
            *query = match items.partition_point(|item| *query >= item[0]) {
                0 => 0,
                i => items[i - 1][1],
            }
        }

        queries
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::maximum_beauty(
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6]
        ),
        vec![2, 4, 5, 5, 6, 6]
    )
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::maximum_beauty(vec![vec![1, 2]], vec![1, 2, 3, 4, 5, 6]),
        vec![2, 2, 2, 2, 2, 2]
    )
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::maximum_beauty(vec![], vec![1, 2, 3, 4, 5, 6]),
        vec![0, 0, 0, 0, 0, 0]
    )
}

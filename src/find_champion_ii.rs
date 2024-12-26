use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut players = (0..n).collect::<HashSet<_>>();

        for edge in edges.iter() {
            players.remove(&edge[1]);
        }

        if players.len() != 1 {
            return -1;
        }

        *players.iter().next().unwrap()
    }

    pub fn find_champion_2(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut players = (0..n).collect::<Vec<_>>();

        for edge in edges.iter() {
            if let Ok(idx) = players.binary_search(&edge[1]) {
                players.remove(idx);
            }
        }

        if players.len() != 1 {
            return -1;
        }

        players[0]
    }

    pub fn find_champion_3(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        macro_rules! impl_with_bits {
            ($n:expr, $edges:expr, $type:ty) => {{
                let mut players: $type = 0;
                for edge in $edges {
                    players |= (1 as $type) << edge[1];
                }
                if players.count_ones() != $n as u32 - 1 {
                    return -1;
                }
                (!players).trailing_zeros() as i32
            }};
        }

        match n {
            0..=8 => impl_with_bits!(n, edges, u8),
            9..=16 => impl_with_bits!(n, edges, u16),
            17..=32 => impl_with_bits!(n, edges, u32),
            33..=64 => impl_with_bits!(n, edges, u64),
            _ => impl_with_bits!(n, edges, u128),
        }
    }

    pub fn find_champion_4(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut defeated = vec![false; n as usize];

        for edge in edges {
            defeated[edge[1] as usize] = true;
        }

        let mut champion = -1;
        let mut champion_count = 0;

        for (idx, is_defeated) in defeated.iter().enumerate() {
            if !is_defeated {
                champion = idx as i32;
                champion_count += 1;
            }
        }

        if champion_count != 1 {
            -1
        } else {
            champion
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::find_champion_3(3, vec![vec![0, 1], vec![1, 2]]),
        0
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::find_champion_3(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]),
        -1
    );
}

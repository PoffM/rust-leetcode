struct Solution {}

impl Solution {
    pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            for num in code.iter_mut() {
                *num = 0;
            }
            return code;
        }

        let n = code.len();

        let mut result = Vec::with_capacity(n);

        if k > 0 {
            for i in 0..n {
                result.push((1..=k).map(|j| code[(i + j as usize) % n]).sum());
            }
        } else {
            for i in 0..n {
                result.push((1..=-k).map(|j| code[(n + i - j as usize) % n]).sum());
            }
        }

        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13])
}

#[test]
fn test_2() {
    assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13])
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::decrypt(vec![10, 5, 7, 7, 3, 2, 10, 3, 6, 9, 1, 6], -4),
        vec![22, 26, 22, 28, 29, 22, 19, 22, 18, 21, 28, 19]
    );
}

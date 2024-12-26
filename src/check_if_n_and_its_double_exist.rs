struct Solution {}

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut nums = [false; 4001];
        let mut has_zero = false;
        for num in arr.iter() {
            if num == &0 {
                if has_zero {
                    return true;
                }
                has_zero = true;
                continue;
            }
            nums[(num + 2000) as usize] = true;
        }

        arr.iter()
            .filter(|&num| num % 2 == 0)
            .any(|num| nums[((num / 2) + 2000) as usize])
    }
}

#[test]
fn test_1() {
    assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
}

#[test]
fn test_2() {
    assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
}

#[test]
fn test_3() {
    assert!(!Solution::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]));
}

#[test]
fn test_4() {
    assert!(Solution::check_if_exist(vec![
        -778, -481, 842, 495, 44, 1000, -572, 977, 240, -116, 673, 997, -958, -539, -964, -187,
        -701, -928, 472, 965, -672, -88, 443, 36, 388, -127, 115, 704, -549, 1000, 998, 291, 633,
        423, 57, -77, -543, 72, 328, -938, -192, 382, 179
    ]));
}

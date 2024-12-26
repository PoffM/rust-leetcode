struct Solution {}

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let [mut start, mut target] = [&start, &target].map(|str| {
            str.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_i, &char)| char != b'_')
        });

        start.all(|(start_i, &start)| {
            if let Some((target_i, &target)) = target.next() {
                start == target
                    && ((start == b'L' && start_i >= target_i)
                        || (start == b'R' && start_i <= target_i))
            } else {
                false
            }
        }) && target.next().is_none()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        Solution::can_change("_L__R__R_".to_string(), "L______RR".to_string()),
        true
    );
}

#[test]
fn test_2() {
    assert_eq!(
        Solution::can_change("R_L_".to_string(), "__LR".to_string()),
        false
    );
}

#[test]
fn test_3() {
    assert_eq!(
        Solution::can_change("_R".to_string(), "R_".to_string()),
        false
    );
}

#[test]
fn test_4() {
    assert_eq!(
        Solution::can_change("_L__R__R_L".to_string(), "L______RR_".to_string()),
        false
    );
}

struct Solution {}

const primes: [i32; 173] = [
    0, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191,
    193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
    307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419,
    421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
    547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
    659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787,
    797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919,
    929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019, 1021,
];

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let mut left_num = nums[0];
        left_num -= Solution::highest_prime_under_input(left_num);

        for right_num in nums.iter().skip(1) {
            let mut smallest_num = *right_num;
            if smallest_num <= left_num {
                return false;
            }
            smallest_num -= Solution::highest_prime_under_input(right_num - left_num);

            left_num = smallest_num;
        }

        true
    }

    fn highest_prime_under_input(input: i32) -> i32 {
        if input <= 0 {
            return 0;
        }

        let mut min = 0;
        let mut max = primes.len() - 2;

        loop {
            let mid = min + ((max - min) / 2);

            let left = primes[mid];
            let right = primes[mid + 1];

            if left < input && right >= input {
                return left;
            }

            if left >= input {
                max -= (max - min) / 2;
            }

            if right <= input {
                min += (max - min) / 2;
            }
        }
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::prime_sub_operation(vec![4, 9, 6, 10]), true)
}

#[test]
fn test_2() {
    assert_eq!(Solution::prime_sub_operation(vec![998, 2]), true)
}

#[test]
fn test_3() {
    assert_eq!(Solution::prime_sub_operation(vec![4, 3, 7, 4]), false)
}

#[test]
fn test_4() {
    assert_eq!(Solution::prime_sub_operation(vec![1]), true)
}

#[test]
fn test_5() {
    assert_eq!(Solution::prime_sub_operation(vec![1, 20, 7, 12, 4]), false)
}

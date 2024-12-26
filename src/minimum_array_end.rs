struct Solution {}

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let n = n as i64;
        let x = x as i64;

        let new_bits = Solution::i64_to_bits(n - 1);
        let mut base_bits = Solution::i64_to_bits(x);

        let mut new_cursor = 0;

        for base_val in base_bits.iter_mut().filter(|base_val| !**base_val) {
            let new_val = new_bits[new_cursor];
            if new_val {
                *base_val = new_val;
            }
            new_cursor += 1;
        }

        Solution::bits_to_i64(&base_bits)
    }

    fn i64_to_bits(num: i64) -> [bool; 64] {
        let mut bits = [false; 64];
        for i in 0..64 {
            bits[i] = (num & (1 << i)) != 0;
        }
        bits
    }

    fn bits_to_i64(bits: &[bool; 64]) -> i64 {
        let mut result: i64 = 0;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                result |= 1 << i;
            }
        }
        result
    }
}

#[test]
fn test_1() {
    assert_eq!(Solution::min_end(3, 4), 6)
}
#[test]
fn test_2() {
    assert_eq!(Solution::min_end(2, 7), 15)
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = format!("^{}%", s);
        let p = format!("^{}%", p);

        let mut cursor: i32 = -1;

        'parts_loop: for p_part in p.split('*').filter(|sub| !sub.is_empty()) {
            if p_part.len() > s.len() {
                return false;
            }

            let mut locations = vec![];
            for si in 0..(s.len() - p_part.len() + 1) {
                if s.chars()
                    .skip(si)
                    .take(p_part.len())
                    .zip(p_part.chars())
                    .all(|(sc, pc)| sc == pc || pc == '?')
                {
                    locations.push((si, si + p_part.len() - 1));
                }
            }
            if locations.is_empty() {
                return false;
            }

            for (low, high) in locations {
                if low as i32 > cursor && high as i32 > cursor {
                    cursor = high as i32;
                    continue 'parts_loop;
                }
            }
            return false;
        }

        true
    }
}

#[test]
fn test1() {
    // test no wildcards
    assert_eq!(
        Solution::is_match("abcde".to_owned(), "abcde".to_owned()),
        true
    );
    assert_eq!(
        Solution::is_match("abde".to_owned(), "abcde".to_owned()),
        false
    );

    // test '?'
    assert_eq!(
        Solution::is_match("abcde".to_owned(), "ab?de".to_owned()),
        true
    );
    assert_eq!(
        Solution::is_match("abde".to_owned(), "ab?de".to_owned()),
        false
    );

    // test '*'
    assert_eq!(
        Solution::is_match("abcdef".to_owned(), "a*f".to_owned()),
        true
    );
    assert_eq!(
        Solution::is_match("abcdef".to_owned(), "a*fz".to_owned()),
        false
    );
    assert_eq!(
        Solution::is_match("abcdef".to_owned(), "a*".to_owned()),
        true
    );

    // assert_eq!(Solution::is_match("aaa".to_owned(), "*a".to_owned()), true);
    assert_eq!(
        Solution::is_match("aza".to_owned(), "a*?a".to_owned()),
        true
    );
    assert_eq!(
        Solution::is_match("azza".to_owned(), "a*?a".to_owned()),
        true
    );

    assert_eq!(
        Solution::is_match("abcabczzzde".to_owned(), "*abc???de*".to_owned()),
        true
    );
}

#[test]
fn test2() {
    assert_eq!(Solution::is_match("".to_owned(), "*****".to_owned()), true);
}

#[test]
fn test4() {
    assert_eq!(
        Solution::is_match("aaaa".to_owned(), "***a".to_owned()),
        true
    );
}
#[test]
fn test5() {
    assert_eq!(
        Solution::is_match("abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_owned(), "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_owned()),
        false
    );
}

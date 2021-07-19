struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }

        if haystack.len() < needle.len() {
            return -1;
        }

        for i in 0..(haystack.len() - needle.len() + 1) {
            if Solution::starts_with(&haystack[i..], needle.as_str()) {
                return i as i32;
            }
        }

        -1
    }

    pub fn starts_with(x: &str, y: &str) -> bool {
        if x.len() < y.len() {
            return false;
        }

        let mut x_chars = x.chars();
        for c in y.chars() {
            if x_chars.nth(0).unwrap() != c {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn str_str() {
        assert_eq!(Solution::str_str("nsalut".to_string(), "sal".to_string()), 1);
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("aaaaa".to_string(), "bb".to_string()), -1);
        assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::str_str("a".to_string(), "a".to_string()), 0);
    }

    #[test]
    fn starts_with() {
        assert_eq!(Solution::starts_with("salut", "sal"), true);
        assert_eq!(Solution::starts_with("nsalut", "sal"), false);
        assert_eq!(Solution::starts_with("salut", "salut1"), false);
        assert_eq!(Solution::starts_with("a", "a"), true);
    }
}

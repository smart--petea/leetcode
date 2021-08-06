struct Solution;
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s_len: usize = s.len();
        let mut j: usize = 0;
        for i in 1..=s_len/2  {
            if s_len % i != 0 {
                continue;
            }

            if s[..i].repeat(s_len/i) == s {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn repeated_substring_pattern() {
        assert_eq!(Solution::repeated_substring_pattern("a".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("ab".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("aa".to_string()), true);
        assert_eq!(Solution::repeated_substring_pattern("abab".to_string()), true);
        assert_eq!(Solution::repeated_substring_pattern("abcab".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("abcab".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("aba".to_string()), false);
        assert_eq!(Solution::repeated_substring_pattern("abcabcabcabc".to_string()), true);
    }
}

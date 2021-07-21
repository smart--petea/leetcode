struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();

        let s_len: usize = s.len();
        let more_than_half: usize = (s_len >> 1) + 1;
        for i in 0..more_than_half {
            let j = s_len - i - 1;
            if  j < i {
                break;
            }

            if s_vec[j] != s_vec[i] {
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
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome("1".to_string()), true);
        assert_eq!(Solution::is_palindrome("amanaplanacanalpanama".to_string()), true);
        assert_eq!(Solution::is_palindrome("raceacar".to_string()), false);
    }
}

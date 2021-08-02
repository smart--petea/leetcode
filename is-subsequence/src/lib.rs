struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        let t_vec: Vec<char> = t.chars().collect::<Vec<char>>();
        let t_len: usize = t.len();

        let mut j: usize = 0;
        for c in s.chars() {
            if j >= t_len {
                return false;
            }

            while j < t_len && t_vec[j] != c {
                j = j + 1;
            }

            if j >= t_len {
                return false;
            }

            j = j + 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_subsequence() {
        //assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
        assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
    }
}

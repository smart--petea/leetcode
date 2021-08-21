struct Solution;
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        let goal_vec: Vec<char> = goal.chars().collect::<Vec<char>>();
        let mut rotated: String = String::new();
        let mut m_len: usize = 0;

        'outer: for i in 0..s_vec.len() {
            for j in 0..(s_vec.len() - i) {
                if s_vec[i + j] != goal_vec[j] {
                    continue 'outer;
                }
            }

            rotated = s_vec[i..].iter().collect::<String>();
            rotated.push_str(&s_vec[..i].iter().collect::<String>());

            if rotated == goal {
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
    fn rotate_string() {
        assert_eq!(Solution::rotate_string("abcde".to_string(), "abcde".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "bcdea".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "cdeab".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "deabc".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "eabcd".to_string()), true);
        assert_eq!(Solution::rotate_string("abcde".to_string(), "abced".to_string()), false);
    }
}

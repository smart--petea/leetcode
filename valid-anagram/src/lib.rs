struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_counts: [usize; 26] = [0; 26];
        let mut t_counts: [usize; 26] = [0; 26];

        let s_chars: Vec<char> = s.chars().collect::<Vec<char>>();
        let t_chars: Vec<char> = t.chars().collect::<Vec<char>>();

        for i in 0..s_chars.len() {
            let s_i = s_chars[i] as usize - 97;
            s_counts[s_i] = s_counts[s_i] + 1;

            let t_i = t_chars[i] as usize - 97;
            t_counts[t_i] = t_counts[t_i] + 1;
        }

        for i in 0..26 {
            if s_counts[i] != t_counts[i] {
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
    fn is_anagram() {
        assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
    }
}

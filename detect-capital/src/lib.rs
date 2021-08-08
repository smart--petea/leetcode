struct Solution;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }

        let is_capital = |x:char| x <= 'Z';
        let mut word_chars = word.chars();
        let first_is_upper: bool = is_capital(word_chars.next().unwrap());
        let mut has_uppers: bool = false;
        let mut has_lowers: bool = false;

        for c in word_chars {
            if is_capital(c) {
                has_uppers = true;
            } else {
                has_lowers = true;
            }

            if has_uppers && has_lowers {
                return false;
            }
        }

        if !first_is_upper && has_uppers {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn detect_capital_use() {
        assert_eq!(Solution::detect_capital_use("abcd".to_string()), true);
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
        assert_eq!(Solution::detect_capital_use("UsA".to_string()), false);
        assert_eq!(Solution::detect_capital_use("uSA".to_string()), false);
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    }
}

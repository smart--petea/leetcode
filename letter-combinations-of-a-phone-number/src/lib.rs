struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let mapping = vec!["0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = vec!["".to_string()];

        for (i, d) in digits.char_indices() {
            let d = d as usize - '0' as usize;

            let mut result1 = Vec::new();
            for s in result {
                for c in mapping[d].chars() {
                    let mut t = s.clone();
                    t.push(c);
                    result1.push(t);
                }
            }

            result = result1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_combinations() {
        let digits = "23".to_string();
        let result = vec!["ad".to_string(), "ae".to_string(), "af".to_string(), "bd".to_string(), "be".to_string(), "bf".to_string(), "cd".to_string(), "ce".to_string(), "cf".to_string()];
        assert_eq!(Solution::letter_combinations(digits), result);

        let digits = "".to_string();
        let result: Vec<String> = Vec::new();
        assert_eq!(Solution::letter_combinations(digits), result);

        let digits = "2".to_string();
        let result = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::letter_combinations(digits), result);
    }
}

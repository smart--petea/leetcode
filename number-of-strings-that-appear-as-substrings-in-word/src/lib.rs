struct Solution;
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut count = 0i32;

        for pattern in patterns {
            if word.contains(&pattern) {
                count += 1;
            }
        }
        
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let patterns = vec!["a".to_string(), "abc".to_string(), "bc".to_string(), "d".to_string()];
        let word = "abc".to_string();
        let expected = 3;
        let output = Solution::num_of_strings(patterns, word);
        assert_eq!(expected, output);

        let patterns = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let word = "aaaabbbb".to_string();
        let expected = 2;
        let output = Solution::num_of_strings(patterns, word);
        assert_eq!(expected, output);
    }
}

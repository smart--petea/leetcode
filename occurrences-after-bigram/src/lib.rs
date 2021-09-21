struct Solution;
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words = text.split(' ').collect::<Vec<&str>>();
        let mut i1 = 0usize;
        let words_count = words.len();

        let mut result = Vec::new();
        while i1 < words_count - 2 {
            if words[i1] == first && words[i1 + 1] == second {
                result.push(words[i1 + 2].to_string());
            }

            i1 = i1 + 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_ocurrences() {
        let text = "alice is a good girl she is a good student".to_string();
        let first = "a".to_string();
        let second = "good".to_string();
        let result = vec!["girl".to_string(), "student".to_string()];
        assert_eq!(Solution::find_ocurrences(text, first, second), result);

        let text = "we will we will rock you".to_string();
        let first = "we".to_string();
        let second = "will".to_string();
        let result = vec!["we".to_string(), "rock".to_string()];
        assert_eq!(Solution::find_ocurrences(text, first, second), result);
    }
}

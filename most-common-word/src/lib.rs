use std::collections::HashSet;
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn most_common_word(paragraph: String, banned_vec: Vec<String>) -> String {
        let mut word_counter: HashMap<String, usize> = HashMap::new();
        let mut banned_set: HashSet<String> = HashSet::new();
        for banned_word in banned_vec {
            banned_set.insert(banned_word);
        }

        for word in paragraph.to_lowercase().replace(&['!', '?', '\'', ',', ';', '.'][..], " ").split(' ') {
            if banned_set.contains(word) {
                continue;
            }

            if word.len() == 0 {
                continue;
            }

            match word_counter.get(word) {
                Some(count) => word_counter.insert(word.to_string(), count + 1),
                None => word_counter.insert(word.to_string(), 1),
            };
        }

        let mut result: String = String::new();
        let mut result_count: usize = 0;

        for (word, count) in word_counter {
            if count > result_count {
                result = word;
                result_count = count;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution; 

    #[test]
    fn most_common_word() {
        let paragraph: String = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
        let banned: Vec<String> = vec!["hit".to_string()];
        let result: String = "ball".to_string();
        assert_eq!(Solution::most_common_word(paragraph, banned), result);

        let paragraph: String = "a".to_string();
        let banned: Vec<String> = Vec::new();
        let result: String = "a".to_string();
        assert_eq!(Solution::most_common_word(paragraph, banned), result);
    }
}



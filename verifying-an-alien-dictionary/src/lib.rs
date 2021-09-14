struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_map: [usize; 26] = [0; 26];
        for (i, c) in order.char_indices() {
            order_map[c as usize - 97usize] = i;
        }

        for i in 0..(words.len() - 1) {
            if Solution::are_ordered(&words[i], &words[i+1], &order_map) == false {
                return false;
            }
        }

        true
    }

    pub fn are_ordered(word1: &String, word2: &String, order: &[usize; 26]) -> bool {
        let word_vec1 = word1.chars().collect::<Vec<char>>();
        let word_vec2 = word2.chars().collect::<Vec<char>>();

        let mut i = 0;
        loop {
            if i >= word_vec1.len() {
                break;
            }

            if i >= word_vec2.len() {
                break;
            }

            let c1 = order[word_vec1[i] as usize - 97usize];
            let c2 = order[word_vec2[i] as usize - 97usize];
            if  c1 == c2 {
                i = i + 1;
                continue;
            }

            if c1 < c2 {
                return true;
            } else {
                return false;
            }

        }

        if word_vec1.len() <= word_vec2.len() {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_ordered() {
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let mut order_map: [usize; 26] = [0; 26];
        for (i, c) in order.char_indices() {
            order_map[c as usize - 97usize] = i;
        }

        let word1 = "hlab".to_string();
        let word2 = "hlab".to_string();
        let result = true;
        assert_eq!(Solution::are_ordered(&word1, &word2, &order_map), result);

        let word1 = "hlba".to_string();
        let word2 = "hlab".to_string();
        let result = false;
        assert_eq!(Solution::are_ordered(&word1, &word2, &order_map), result);

        let word1 = "hla".to_string();
        let word2 = "hlab".to_string();
        let result = true;
        assert_eq!(Solution::are_ordered(&word1, &word2, &order_map), result);

        let word1 = "hlab".to_string();
        let word2 = "hla".to_string();
        let result = false;
        assert_eq!(Solution::are_ordered(&word1, &word2, &order_map), result);

        let word1 = "hello".to_string();
        let word2 = "leetcode".to_string();
        let result = true;
        assert_eq!(Solution::are_ordered(&word1, &word2, &order_map), result);
    }

    #[test]
    fn is_alien_sorted() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        let result = true;
        assert_eq!(Solution::is_alien_sorted(words, order), result);

        let words = vec!["word".to_string(),"world".to_string(),"row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        let result = false;
        assert_eq!(Solution::is_alien_sorted(words, order), result);

        let words = vec!["apple".to_string(),"app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result = false;
        assert_eq!(Solution::is_alien_sorted(words, order), result);
    }
}

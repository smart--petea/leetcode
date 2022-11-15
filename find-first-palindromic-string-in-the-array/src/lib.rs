struct Solution;
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if Self::is_palindrome(&word) {
                return word;
            }
        }

        String::new()
    }

    pub fn is_palindrome(word: &String) -> bool {
        if word.len() == 0 {
            return true;
        }

        if word.len() == 1 {
            return true;
        }

        let mut i = 0;
        let mut j = word.len() - 1;
        let word_bytes = word.as_bytes();

        while j > 0 && i <= j && word_bytes[i] == word_bytes[j] {
            i = i + 1;
            j = j - 1;
        }

        i > j
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_palindrome() {
        let words = vec!["abc".to_string(),"car".to_string(),"ada".to_string(),"racecar".to_string(),"cool".to_string()];
        let output = Solution::first_palindrome(words);
        let expected = "ada".to_string();
        assert_eq!(output, expected);

        let words = vec!["notapalindrome".to_string(),"racecar".to_string()];
        let output = Solution::first_palindrome(words);
        let expected = "racecar".to_string();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_is_palindrome() {
        /*
        let word = "".to_string();
        assert_eq!(true, Solution::is_palindrome(&word));

        let word = "a".to_string();
        assert_eq!(true, Solution::is_palindrome(&word));

        let word = "aa".to_string();
        assert_eq!(true, Solution::is_palindrome(&word));

        let word = "ab".to_string();
        assert_eq!(false, Solution::is_palindrome(&word));

        let word = "aba".to_string();
        assert_eq!(true, Solution::is_palindrome(&word));

        let word = "abab".to_string();
        assert_eq!(false, Solution::is_palindrome(&word));

        let word = "abba".to_string();
        assert_eq!(false, Solution::is_palindrome(&word));
        */

        let word = "racecar".to_string();
        assert_eq!(true, Solution::is_palindrome(&word));
    }
}

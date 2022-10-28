struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        if sentence.len() < 26 {
            return false;
        }

        let full_alphabet = (1 << 26) - 1;
        let mut occurences = 0;

        for c in sentence.bytes() {
            occurences |= 1 << (c - b'a');

            if occurences == full_alphabet {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        let expected = true;
        let output = Solution::check_if_pangram(sentence);
        assert_eq!(output, expected);

        let sentence = "leetcode".to_string();
        let expected = false;
        let output = Solution::check_if_pangram(sentence);
        assert_eq!(output, expected);
    }
}

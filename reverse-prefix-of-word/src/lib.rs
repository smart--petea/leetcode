struct Solution;
impl Solution {
    pub fn reverse_prefix(mut word: String, ch: char) -> String {
        if let Some(position) = word.find(ch) {
            unsafe {
                word.as_mut_vec()[..=position].reverse();
            }
        }

        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "abcdefd".to_string();
        let ch = 'd';
        let expected =  "dcbaefd".to_string();
        let output = Solution::reverse_prefix(word, ch);

        let word = "xyxzxe".to_string();
        let ch = 'z';
        let expected =  "zxyxxe".to_string();
        let output = Solution::reverse_prefix(word, ch);

        let word = "abcd".to_string();
        let ch = 'z';
        let expected =  "abcd".to_string();
        let output = Solution::reverse_prefix(word, ch);
    }
}

struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken_letters_vec = [false; 26];
        for c in broken_letters.chars() {
            broken_letters_vec[c as usize - 97usize] = true;
        }

        let mut result = 0i32;
        'outer: for word in text.split(' ') {
            for c in word.chars() {
                if broken_letters_vec[c as usize - 97usize] {
                    continue 'outer;
                }
            }

            result += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_typed_words() {
        let text = "leet code".to_string();
        let broken_letters = "lt".to_string();
        let result = 1i32;

        assert_eq!(Solution::can_be_typed_words(text, broken_letters), result);

        let text = "leet code".to_string();
        let broken_letters = "e".to_string();
        let result = 0i32;

        assert_eq!(Solution::can_be_typed_words(text, broken_letters), result);
    }
}

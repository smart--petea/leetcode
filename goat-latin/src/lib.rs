struct Solution;
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut result: String = String::new(); 

        for (i, word) in sentence.split(' ').enumerate() {
            if i > 0 {
                result.push(' ');
            }

            match word.chars().next() {
                Some(c) if Solution::is_vowel(&c) => {
                    result.push_str(word);
                }
                Some(c) => {
                    result.push_str(&word[1..]);
                    result.push(c);
                }
                None => ()
            }

            result.push_str("ma");
            result.push_str(&"a".repeat(i + 1));
        }

        result
    }

    pub fn is_vowel(c: &char) -> bool {
        match c {
        'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => true,
        _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn to_goat_latin() {
        let sentence: String = "I speak Goat Latin".to_string();
        let result: String = "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string();
        assert_eq!(Solution::to_goat_latin(sentence), result);

        let sentence: String = "The quick brown fox jumped over the lazy dog".to_string();
        let result: String = "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string();
        assert_eq!(Solution::to_goat_latin(sentence), result);
    }
}

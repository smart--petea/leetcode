use std::collections::HashMap;

impl Solution {
  pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut word_iter = s.split_whitespace();

        let mut letter_to_word: HashMap::<char, String> = HashMap::<char, String>::new();
        let mut word_to_letter: HashMap::<String, char> = HashMap::<String, char>::new();

        for c in pattern.chars() {
            if c == ' ' {
                continue;
            }

            let word: String = match word_iter.next() {
                Some(word) => word.to_string(),
                None => {
                    return false;
                }
            };

            match letter_to_word.get(&c) {
                Some(word1) => {
                    if word != *word1 {
                        return false;
                    }
                    continue;
                }
                None => (),
            }

            match word_to_letter.get(&word) {
                Some(_c1) => {
                    return false;
                }
                None => (),
            }

            letter_to_word.insert(c, word.clone());
            word_to_letter.insert(word.clone(), c);
        }

        if let Some(_word) = word_iter.next() {
            return false;
        }

        true
    }
}

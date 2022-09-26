use std::slice::Iter;

struct Solution;

struct LetterIterator<'a> {
    vec_iterator: Iter<'a, String>,
    u8_iterator: Iter<'a, u8>
}

impl <'a> LetterIterator<'a> {
    const EMPTY_ARRAY: [u8; 0] = [];
    fn new(word: &'a Vec<String>) -> Self {
        let mut vec_iterator = word.iter();
        let u8_iterator = match vec_iterator.next() {
            Some(word_item) => word_item.as_bytes().into_iter(),
            None => Self::EMPTY_ARRAY[..].into_iter(),
        };

        LetterIterator {
            vec_iterator: vec_iterator,
            u8_iterator: u8_iterator,
        }
    }
}


impl <'a> Iterator for LetterIterator<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.u8_iterator.next() {
            c @ Some(_) => {
                return c;
            }
            None => {}
        }

        match self.vec_iterator.next() {
            Some(v) => {
                self.u8_iterator = v.as_bytes().into_iter();
                return self.next();
            }

            None => {
                return None;
            }
        }
    }
}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut it1 = LetterIterator::new(&word1);
        let mut it2 = LetterIterator::new(&word2);

        it1.eq(it2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterating_letter_iterator() {
        let input = vec!["sal".to_string(), "ut".to_string(), "!".to_string()];
        let mut it = LetterIterator::new(&input);

        assert_eq!(it.next(), Some(&b's'));
        assert_eq!(it.next(), Some(&b'a'));
        assert_eq!(it.next(), Some(&b'l'));
        assert_eq!(it.next(), Some(&b'u'));
        assert_eq!(it.next(), Some(&b't'));
        assert_eq!(it.next(), Some(&b'!'));
        assert_eq!(it.next(), None);

        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn provided_tests() {
        let word1 = vec!["ab".to_string(), "c".to_string()];
        let word2 = vec!["a".to_string(), "bc".to_string()];
        let expected = true;
        let output = Solution::array_strings_are_equal(word1, word2);
        assert_eq!(expected, output);

        let word1 = vec!["a".to_string(), "cb".to_string()];
        let word2 = vec!["ab".to_string(), "c".to_string()];
        let expected = false;
        let output = Solution::array_strings_are_equal(word1, word2);
        assert_eq!(expected, output);

        let word1  = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
        let word2 = vec!["abcddefg".to_string()];
        let expected = true;
        let output = Solution::array_strings_are_equal(word1, word2);
        assert_eq!(expected, output);
    }
}

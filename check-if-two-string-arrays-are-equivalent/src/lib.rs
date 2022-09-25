use std::slice::Iter;

struct Solution;

struct LetterIterator<'a> {
    vec_iterator: Iter<'a, String>,
    bytes_iterator: Iter<'a, u8>
}

impl <'a> LetterIterator<'a> {
    const EMPTY_ARRAY: [u8; 0] = [];
    fn new(word: &'a Vec<String>) -> Self {
        let mut vec_iterator = word.iter();
        let bytes_iterator = match vec_iterator.next() {
            Some(word_item) => word_item.as_bytes().into_iter(),
            None => Self::EMPTY_ARRAY[..].into_iter(),
        };

        LetterIterator {
            vec_iterator: vec_iterator,
            bytes_iterator: bytes_iterator,
        }
    }
}


/*
impl <'a> Iterator for LetterIterator<'a> {
    type Item = char;

    fn next(&'a mut self) -> Option<Self::Item> {
        
    }
}
*/

impl Solution {
    /*
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        
    }
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

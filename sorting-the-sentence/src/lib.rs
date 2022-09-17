struct Solution;
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words: [Option<&str>;10] = [None;10];

        for word in s.split_whitespace() {
            let position = word.chars().last().unwrap().to_digit(10).unwrap() as usize ;
            words[position] = Some(&word[..word.len() - 1]);
        }

        let mut result = String::new();
        let mut iter = words.into_iter().enumerate();
        iter.next();//skip 0 element
        while let Some((position, Some(s)))  = iter.next() {
            if position != 1 {
                result.push(' ');
            }

            result.push_str(s);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "is2 sentence4 This1 a3".to_string();
        let result = Solution::sort_sentence(input);
        let expected = "This is a sentence".to_string();
        assert_eq!(expected, result);
    }
}

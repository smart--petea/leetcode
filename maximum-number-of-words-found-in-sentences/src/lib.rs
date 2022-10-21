struct Solution;
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
       sentences.iter().fold(1, |max_acc, sentence| {
           sentence
               .chars()
               .into_iter()
               .fold(1, |space_counter, chr| {
                   if chr == ' ' {
                       return space_counter + 1;
                   }

                   space_counter
                })
                .max(max_acc)
       })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sentences = vec!["alice and bob love leetcode".to_string(), "i think so too".to_string(), "this is great thanks very much".to_string()];
        let output = Solution::most_words_found(sentences);
        let expected = 6;
        assert_eq!(output, expected);
    }
}

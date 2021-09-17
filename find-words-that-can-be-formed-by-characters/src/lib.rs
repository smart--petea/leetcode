struct Solution;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut freqs = [0usize;26];
        let mut i: usize;
        for c in chars.chars() {
            i  = c as usize - 97usize;
            freqs[i] = freqs[i] + 1;
        }

        let mut word_freqs = [0usize;26];
        let mut result = 0i32;
        'outer:for word in words {
            word_freqs.copy_from_slice(&freqs[..]);

            for c in word.chars() {
                i = c as usize - 97usize;
                if word_freqs[i] == 0 {
                    continue 'outer;
                }

                word_freqs[i] = word_freqs[i] - 1;
            }

            result = result + word.len() as i32;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_characters() {
        let words = vec!["cat".to_string(),"bt".to_string(),"hat".to_string(),"tree".to_string()];
        let chars = "atach".to_string();
        let result = 6i32;
        assert_eq!(Solution::count_characters(words, chars), result);

        let words = vec!["hello".to_string(),"world".to_string(),"leetcode".to_string()];
        let chars = "welldonehoneyr".to_string();
        let result = 10i32;
        assert_eq!(Solution::count_characters(words, chars), result);
    }
}

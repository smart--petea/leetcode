struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut frequencies = [std::i32::MAX; 26];

        for word in words {
            let mut local_frequencies = [0; 26];
            for c in word.chars() {
                let index = c as usize - 97usize;
                local_frequencies[index] = local_frequencies[index] + 1;
            }

            for i in 0..26 {
                frequencies[i] = std::cmp::min(frequencies[i], local_frequencies[i]);
            }
        }

        let mut result = vec![];
        for i in 0..26 {
            while frequencies[i] > 0 {
                frequencies[i] = frequencies[i] - 1;
                result.push(((i as u8 + 97u8) as char).to_string());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_chars() {
        let words = vec!["bella".to_string(),"label".to_string(),"roller".to_string()];
        let result = vec!["e", "l", "l"];
        assert_eq!(Solution::common_chars(words), result);

        let words = vec!["cool".to_string(),"lock".to_string(),"cook".to_string()];
        let result = vec!["c", "o"];
        assert_eq!(Solution::common_chars(words), result);
    }
}

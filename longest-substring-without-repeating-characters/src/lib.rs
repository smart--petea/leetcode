struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut cache = [0usize; 256];
        let mut result = 0i32;

        let mut j = 1usize;
        for (i, c) in s.char_indices() {
            j = if cache[c as usize] > 0 {
                std::cmp::max(j, cache[c as usize] + 1)
            } else {
                j
            };
            cache[c as usize] = (i + 1)as usize;

            result = std::cmp::max(result, i as i32 - j as i32 + 2i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring() {
        let s = "abcabcbb".to_string();
        let result = 3; 
        assert_eq!(Solution::length_of_longest_substring(s), result);

        let s = "bbbbbbbb".to_string();
        let result = 1; 
        assert_eq!(Solution::length_of_longest_substring(s), result);

        let s = "pwwkew".to_string();
        let result = 3; 
        assert_eq!(Solution::length_of_longest_substring(s), result);

        let s = "".to_string();
        let result = 0; 
        assert_eq!(Solution::length_of_longest_substring(s), result);
    }
}

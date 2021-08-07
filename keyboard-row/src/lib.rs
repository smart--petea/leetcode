struct Solution;
impl Solution {
    const FIRST_ROW: [char; 10]  = ['y', 'w','u','t','r','q','p','o','i','e'];
    const SECOND_ROW: [char; 9] = ['s','l','k','j','h','g','f','d','a'];
    const THIRD_ROW: [char; 7] = ['z','x','v','n','m','c','b'];

    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for word in words {
            if Solution::one_row_generated(&word) {
                result.push(word);
            }
        }

        result
    }

    pub fn one_row_generated(word: &String) -> bool {
        let mut counts: [usize; 26] = [0; 26];
        for c in word.to_lowercase().chars() {
            counts[c as usize - 97usize] = 1;
        }

        let mut print: String = String::new();
        for i in 0..26 {
            if counts[i] == 1 {
                print.insert(0, (i as u8 + 97u8) as char);
            }
        }

        if Solution::is_from_pattern(&Solution::FIRST_ROW[..], &print) {
            return true;
        }

        if Solution::is_from_pattern(&Solution::SECOND_ROW[..], &print) {
            return true;
        }

        if Solution::is_from_pattern(&Solution::THIRD_ROW[..], &print) {
            return true;
        }

        false
    }

    pub fn is_from_pattern(pattern: &[char], print: &String) -> bool {
        let pattern_len: usize = pattern.len();
        if print.len() > pattern_len {
            return false;
        }

        let mut j: usize = 0;
        for c in print.to_string().chars() {
            while j < pattern_len && pattern[j] != c {
                j = j + 1;
            }

            if j >= pattern_len {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_words() {
        assert_eq!(Solution::find_words(vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()]), vec!["Alaska".to_string(),"Dad".to_string()]);

        let v: Vec<String> = Vec::new();
        assert_eq!(Solution::find_words(vec!["omk".to_string()]), v);
        assert_eq!(Solution::find_words(vec!["adsdf".to_string(),"sfd".to_string()]), vec!["adsdf".to_string(),"sfd".to_string()]);
    }

    #[test]
    fn is_from_pattern() {
        assert_eq!(Solution::is_from_pattern(&Solution::FIRST_ROW, &"e".to_string()), true);
        assert_eq!(Solution::is_from_pattern(&Solution::FIRST_ROW, &"ey".to_string()), true);
        assert_eq!(Solution::is_from_pattern(&Solution::FIRST_ROW, &"eay".to_string()), false);
    }

    #[test]
    fn one_row_generated() {
        assert_eq!(Solution::one_row_generated(&"Hello".to_string()), false);
        assert_eq!(Solution::one_row_generated(&"Alaska".to_string()), true);
        assert_eq!(Solution::one_row_generated(&"Dad".to_string()), true);
        assert_eq!(Solution::one_row_generated(&"Peace".to_string()), false);

        assert_eq!(Solution::one_row_generated(&"qwertyuiop".to_string()), true);
        assert_eq!(Solution::one_row_generated(&"asdfghjkl".to_string()), true);
        assert_eq!(Solution::one_row_generated(&"zxcvbnm".to_string()), true);
    }
}

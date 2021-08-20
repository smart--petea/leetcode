struct Solution;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let license_print: [usize; 26] = Solution::get_print(&license_plate);

        let mut w: String = String::new(); 
        for word in words {
            if w.len() > 0 && w.len() <= word.len() {
                continue;
            }

            let word_print: [usize; 26] = Solution::get_print(&word);

            if Solution::print1_cover_print2(&word_print, &license_print) {
                w = word;
            }
        }

        w
    }

    pub fn print1_cover_print2(print1: &[usize; 26], print2: &[usize; 26]) -> bool {
        for i in 0..26 {
            if print1[i] < print2[i] {
                return false;
            }
        }

        true
    }

    pub fn get_print(s: &String) -> [usize; 26] {
        let mut print: [usize; 26] = [0; 26];

        for c in s.chars() {
            if c >= 'a' && c <= 'z' {
                print[c as usize - 97] = print[c as usize - 97] + 1;
            } else if c >= 'A' && c <= 'Z' {
                print[c as usize - 65] = print[c as usize - 65] + 1;
            }
        }

        print
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn shortest_completing_word() {
        let license_plate: String = "1s3 PSt".to_string();
        let words: Vec<String> = vec![
            "step".to_string(),
            "steps".to_string(),
            "stripe".to_string(),
            "stepple".to_string(),
        ];

        assert_eq!(Solution::shortest_completing_word(license_plate, words), "steps".to_string());

        let license_plate: String = "1s3 456".to_string();
        let words: Vec<String> = vec![
            "looks".to_string(),
            "pest".to_string(),
            "stew".to_string(),
            "show".to_string(),
        ];

        assert_eq!(Solution::shortest_completing_word(license_plate, words), "pest".to_string());

        let license_plate: String = "Ah71752".to_string();
        let words: Vec<String> = vec![
            "suggest".to_string(),
            "letter".to_string(),
            "of".to_string(),
            "husband".to_string(),
            "easy".to_string(),
            "education".to_string(),
            "drug".to_string(),
            "prevent".to_string(),
            "writer".to_string(),
            "old".to_string(),
        ];

        assert_eq!(Solution::shortest_completing_word(license_plate, words), "husband".to_string());
    }

    #[test]
    fn get_print() {
        let mut result: [usize; 26] = [0; 26];
        result['a' as usize - 97usize] = 1;
        result['b' as usize - 97usize] = 1;
        result['c' as usize - 97usize] = 3;
        result['d' as usize - 97usize] = 3;
        result['f' as usize - 97usize] = 1;
        result['e' as usize - 97usize] = 1;
        result['z' as usize - 97usize] = 1;

        assert_eq!(Solution::get_print(&"abcdccddefz".to_string()), result);
    }

    #[test]
    fn print1_covers_print2() {
        let mut print1: [usize; 26] = [0; 26];
        print1[4] = 3;
        print1[5] = 10;
        print1[15] = 4;

        let mut print2: [usize; 26] = [0; 26];
        print2[5] = 10;
        print2[15] = 3;

        assert_eq!(Solution::print1_cover_print2(&print1, &print2), true);

        print2[5] = 11;
        assert_eq!(Solution::print1_cover_print2(&print1, &print2), false);
    }
}

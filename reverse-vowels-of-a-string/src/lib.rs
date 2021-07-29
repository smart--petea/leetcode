struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut i: usize = 0;
        let mut j: usize = s_vec.len() - 1;

        loop {
            while i < j && Solution::is_vowel(&s_vec[i]) == false {
                i = i + 1;
            }

            if i >= j {
                break;
            }

            while i < j && Solution::is_vowel(&s_vec[j]) == false {
                j = j - 1;
            }

            if i >= j {
                break;
            }

            let c: char = s_vec[i];
            s_vec[i] = s_vec[j];
            s_vec[j] = c;

            i = i + 1;
            j = j - 1;
        }

        s_vec.into_iter().collect::<String>()
    }

    pub fn is_vowel(c: &char) -> bool {
        match *c {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn reverse_vowels() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    }
}

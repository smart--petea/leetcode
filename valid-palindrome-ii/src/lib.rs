struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        if s_vec.len() == 0 {
            return true;
        }

        let mut i: usize = 0;
        let mut j: usize = s_vec.len() - 1;

        while i < j && s_vec[i] == s_vec[j] {
            i = i + 1;
            j = j - 1;
        }

        if i >= j {
            return true;
        }

        let mut i1 = i + 1;
        let mut j1 = j;

        while i1 < j1 && s_vec[i1] == s_vec[j1] {
            i1 = i1 + 1;
            j1 = j1 - 1;
        }

        if i1 >= j1 {
            return true;
        }

        let mut i1 = i;
        let mut j1 = j - 1;
        while i1 < j1 && s_vec[i1] == s_vec[j1] {
            i1 = i1 + 1;
            j1 = j1 - 1;
        }
        if i1 >= j1 {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn valid_palindrome() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("a".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
        assert_eq!(Solution::valid_palindrome("eeccccbebaeeabebccceea".to_string()), false);
        assert_eq!(Solution::valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()), true);
    }
}

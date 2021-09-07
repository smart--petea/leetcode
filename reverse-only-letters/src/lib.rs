struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>(); 
        let mut i: usize = 0;
        let mut j: usize = s_vec.len() - 1;
        let mut temp: char = 'a';

        while i < j {

            while i < j && Solution::is_letter(s_vec[i]) == false {
                i = i + 1;
            }

            while i < j && Solution::is_letter(s_vec[j]) == false {
                j = j - 1;
            }

            if i < j {
                temp = s_vec[i];
                s_vec[i] = s_vec[j];
                s_vec[j] = temp;
            }

            i = i + 1;
            j = j - 1;
        }

        s_vec.iter().collect::<String>()
    }

    pub fn is_letter(c: char) -> bool {
        (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn reverse_only_letters() {
        let s: String = "a".to_string();
        let result: String = "a".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "-".to_string();
        let result: String = "-".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "-a".to_string();
        let result: String = "-a".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "b-a".to_string();
        let result: String = "a-b".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "*ba-".to_string();
        let result: String = "*ab-".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "ab-cd".to_string();
        let result: String = "dc-ba".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "a-bC-dEf-ghIj".to_string();
        let result: String = "j-Ih-gfE-dCba".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);

        let s: String = "Test1ng-Leet=code-Q!".to_string();
        let result: String = "Qedo1ct-eeLg=ntse-T!".to_string();
        assert_eq!(Solution::reverse_only_letters(s), result);
    }
}

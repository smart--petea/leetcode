struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_vec: Vec<char> = a.chars().collect::<Vec<char>>();
        let b_vec: Vec<char> = b.chars().collect::<Vec<char>>();

        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let min_len: usize = std::cmp::min::<usize>(a.len(), b.len());
        let max_len: usize = std::cmp::max::<usize>(a.len(), b.len());
        let mut result: String = String::new();

        let mut zz: char = '0';
        for i in 0..min_len {
            let a_char: char = a_vec[a_len - i - 1];
            let b_char: char = b_vec[b_len - i - 1];
            let (result_char, new_zz) = Solution::add_chars(a_char, b_char, zz);
            zz=new_zz;

            result.insert(0, result_char);
        }

        let max_vec: Vec<char> = if a_len >= b_len { a_vec } else { b_vec };
        let max_len: usize = max_vec.len();
        for i in min_len..max_len {
            let max_char = max_vec[max_len - i - 1];
            let (result_char, new_zz) = Solution::add_chars(max_char, '0', zz);
            zz=new_zz;

            result.insert(0, result_char);
        }

        if zz == '1' {
            result.insert(0, zz);
        }

        result
    }

    pub fn add_chars(a: char, b: char, c: char) -> (char, char) {
        match (a, b, c) {
            ('0', '0', '0') => ('0', '0'), 
            ('0', '0', '1') => ('1', '0'), 
            ('0', '1', '0') => ('1', '0'), 
            ('0', '1', '1') => ('0', '1'), 
            ('1', '0', '0') => ('1', '0'), 
            ('1', '0', '1') => ('0', '1'), 
            ('1', '1', '0') => ('0', '1'), 
            _ => ('1', '1'), 
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn add_binary() {
        assert_eq!(Solution::add_binary("0".to_string(), "0".to_string()), "0".to_string());
        assert_eq!(Solution::add_binary("1".to_string(), "0".to_string()), "1".to_string());
        assert_eq!(Solution::add_binary("1".to_string(), "1".to_string()), "10".to_string());
        assert_eq!(Solution::add_binary("10".to_string(), "01".to_string()), "11".to_string());
        assert_eq!(Solution::add_binary("1".to_string(), "11".to_string()), "100".to_string());
        assert_eq!(Solution::add_binary("1".to_string(), "11".to_string()), "100".to_string());
        assert_eq!(Solution::add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
    }

    #[test]
    fn add_chars() {
        assert_eq!(Solution::add_chars('0', '0', '0'), ('0', '0')); 
        assert_eq!(Solution::add_chars('0', '0', '1'), ('1', '0')); 
        assert_eq!(Solution::add_chars('0', '1', '0'), ('1', '0')); 
        assert_eq!(Solution::add_chars('0', '1', '1'), ('0', '1')); 
        assert_eq!(Solution::add_chars('1', '0', '0'), ('1', '0')); 
        assert_eq!(Solution::add_chars('1', '0', '1'), ('0', '1')); 
        assert_eq!(Solution::add_chars('1', '1', '0'), ('0', '1')); 
        assert_eq!(Solution::add_chars('1', '1', '1'), ('1', '1')); 
    }
}

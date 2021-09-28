struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let s_vec = s.chars().collect::<Vec<char>>();

        let mut max_len = 0usize;
        let mut max_len1 = 0usize;
        let mut start = 0usize;

        for i in 0..(s.len()-1) {
            max_len1 = Solution::max_palindrome(&s_vec, i, i);
            if max_len1 > max_len {
                max_len = max_len1;
                start = i - (max_len-1)/2;
            }

            max_len1 = Solution::max_palindrome(&s_vec, i, i+1);
            if max_len1 > max_len {
                max_len = max_len1;
                start = i - (max_len-2)/2;
            }
        }

        s_vec[start..(start + max_len)].iter().collect::<String>()
    }

    pub fn max_palindrome(v: &Vec<char>, i: usize, j: usize) -> usize {
        if v[i] != v[j] {
            return 0;
        }

        let mut k1 = i;
        let mut k2 = j;

        loop {
            if v[k1] != v[k2] {
                return  k2 - k1 - 1;
            }

            if k1 == 0 ||  k2 == v.len()-1 {
                return k2 - k1 + 1;
            }

            k1 = k1 - 1;
            k2 = k2 + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome() {
        let s = "babad".to_string();
        let result = "bab".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = "cbbd".to_string();
        let result = "bb".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = "a".to_string();
        let result = "a".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = "ac".to_string();
        let result = "a".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);

        let s = "ccc".to_string();
        let result = "ccc".to_string();
        assert_eq!(Solution::longest_palindrome(s), result);
    }

    #[test]
    fn max_palindrome() {
        let v = vec!['a'];
        let i = 0usize;
        let j = 0usize;
        let result = 1;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'a'];
        let i = 0usize;
        let j = 1usize;
        let result = 2;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'b'];
        let i = 0usize;
        let j = 0usize;
        let result = 1;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'b'];
        let i = 0usize;
        let j = 1usize;
        let result = 0;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'b', 'a'];
        let i = 1usize;
        let j = 1usize;
        let result = 3;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'b', 'a'];
        let i = 1usize;
        let j = 2usize;
        let result = 0;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['a', 'b', 'b', 'a'];
        let i = 1usize;
        let j = 2usize;
        let result = 4;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);

        let v = vec!['c', 'b', 'b', 'a'];
        let i = 1usize;
        let j = 2usize;
        let result = 2;
        assert_eq!(Solution::max_palindrome(&v, i, j), result);
    }
}

struct Solution;
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut m = 0;
        let s_vec_len = s_vec.len() - 1;
        let mut tmp: char = 'a';
        let mut k1: usize = 0;
        let mut k2: usize = 0;
        for i in (0..=s_vec_len).step_by(2*k as usize) {
            m = std::cmp::min(i + (k - 1) as usize, s_vec_len);
            k1 = i;
            k2 = m;
            while k1 < k2 {
                tmp = s_vec[k1 as usize];
                s_vec[k1] = s_vec[k2];
                s_vec[k2] = tmp;

                k1 = k1 + 1;
                k2 = k2 - 1;
            }
        }

        s_vec.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn reverse_str() {
        assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg".to_string());
        assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd".to_string());
        assert_eq!(Solution::reverse_str("abcdefgabcdefg".to_string(), 3), "cbadefbagcdegf".to_string());
    }
}

struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        let s_vec_len: usize = s_vec.len();
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        let mut tmp: char = 'a';
        let mut j: usize = 0;

        while i1 < s_vec_len {
            i2 = i1;
            while i2 < s_vec_len && s_vec[i2] != ' ' {
                i2 = i2 + 1;
            }

            j = 0;
            while i1 + j < i2 - j {
                tmp = s_vec[i1 + j];
                s_vec[i1 + j] = s_vec[i2 - j - 1]; 
                s_vec[i2 - j - 1] = tmp;
                j = j + 1;
            }

            i1 = i2 + 1;
        }

        s_vec.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution; 

    #[test]
    fn reverse_words() {
        assert_eq!(Solution::reverse_words("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string());
        assert_eq!(Solution::reverse_words("God Ding".to_string()), "doG gniD".to_string());
        assert_eq!(Solution::reverse_words("Ding".to_string()), "gniD".to_string());
    }
}




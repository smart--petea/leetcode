struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut zeros: i32 = 0; 
        let mut ones: i32 = 0;

        let mut prev: char = 'a';
        let mut cnt: i32 = 0; 

        for c in s.chars() {
            if c != prev {
                cnt = cnt + std::cmp::min(zeros, ones);

                if c == '0' {
                   zeros = 0; 
                } else {
                   ones = 0; 
                }
                prev = c;
            }

            if c == '0' {
                zeros = zeros + 1;
            } else {
                ones = ones + 1;
            }
        }  

        cnt + std::cmp::min(zeros, ones)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn count_binary_substrings() {
        assert_eq!(Solution::count_binary_substrings("01".to_string()), 1);
        assert_eq!(Solution::count_binary_substrings("101".to_string()), 2);
        assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
        assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
        assert_eq!(Solution::count_binary_substrings("00100".to_string()), 2);
    }
}



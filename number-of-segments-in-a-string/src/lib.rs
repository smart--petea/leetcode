struct Solution;
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut cnt: i32 = 0;
        let mut i: usize = 0;

        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        let s_len: usize = s.len();

        loop {
            if i >= s_len {
                break;
            }

            while i < s_len && s_vec[i] == ' ' {
                i = i + 1;
            }

            if i < s_len {
                cnt = cnt+1;
            }

            while i < s_len && s_vec[i] != ' ' {
                i = i + 1;
            }
        }

        cnt
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn count_segments() {
        assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
        assert_eq!(Solution::count_segments("Hello".to_string()), 1);
        assert_eq!(Solution::count_segments("love live! mu'sic forever".to_string()), 4);
        assert_eq!(Solution::count_segments("".to_string()), 0);
    }
}

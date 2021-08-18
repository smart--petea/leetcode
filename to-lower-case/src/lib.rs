struct Solution;
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        for i in 0..s_vec.len() {
            if s_vec[i] >= 'A' && s_vec[i] <= 'Z' {
                s_vec[i] = (s_vec[i] as u8 + 32u8) as char;
            }
        }

        s_vec.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn to_lower_case() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello".to_string());
        assert_eq!(Solution::to_lower_case("here".to_string()), "here".to_string());
        assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely".to_string());
    }
}

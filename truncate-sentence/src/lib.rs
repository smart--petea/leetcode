struct Solution;
impl Solution {
    pub fn truncate_sentence(s: String, mut k: i32) -> String {
        let mut boundary = 0;
        for (i, c) in s.chars().into_iter().enumerate() {
            boundary = i;
            if c == ' ' {
                k = k - 1;
                if k == 0 {
                    boundary = boundary - 1;
                    break;
                }
            }
        }

        s[0..=boundary].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello how are you Contestant".to_string();
        let k = 4;
        let expected = "Hello how are you".to_string();
        let output = Solution::truncate_sentence(s, k);
        assert_eq!(expected, output);

        let s = "What is the solution to this problem".to_string();
        let k = 4;
        let expected = "What is the solution".to_string();
        let output = Solution::truncate_sentence(s, k);
        assert_eq!(expected, output);

        let s = "chopper is not a tanuki".to_string();
        let k = 5;
        let expected = "chopper is not a tanuki".to_string();
        let output = Solution::truncate_sentence(s, k);
        assert_eq!(expected, output);
    }
}

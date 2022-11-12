struct Solution;
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut result = String::new();
        let mut b_rev_iter = s.bytes().rev();

        while let Some(el) = b_rev_iter.next() {
            if el == b'#' {
                let second = b_rev_iter.next().unwrap() - b'0';
                let first = b_rev_iter.next().unwrap() - b'0';

                result.push((first * 10 + second + 96) as char);
            } else {
                result.push((el + 48) as char);
            }
        }

        result.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "10#11#12".to_string();
        let expected = "jkab".to_string();
        let output = Solution::freq_alphabets(input);
        assert_eq!(output, expected);

        let input = "1326#".to_string();
        let expected = "acz".to_string();
        let output = Solution::freq_alphabets(input);
        assert_eq!(output, expected);
    }
}

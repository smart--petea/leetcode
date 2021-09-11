struct Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let size: usize = s.len();
        let mut result: Vec<i32> = Vec::with_capacity(size + 1);

        let mut low: i32 = 0;
        let mut high: i32 = size as i32;

        for c in s.chars() {
            if c == 'I' {
                result.push(low);
                low = low + 1;
            } else {
                result.push(high);
                high = high - 1;
            }
        }
        result.push(high);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn di_string_match() {
        let s: String = "IDID".to_string();
        let result: Vec<i32> = vec!(0, 4, 1, 3, 2);
        assert_eq!(Solution::di_string_match(s), result);

        let s: String = "III".to_string();
        let result: Vec<i32> = vec!(0, 1, 2, 3);
        assert_eq!(Solution::di_string_match(s), result);

        let s: String = "DDI".to_string();
        let result: Vec<i32> = vec!(3, 2, 0, 1);
        assert_eq!(Solution::di_string_match(s), result);
    }
}

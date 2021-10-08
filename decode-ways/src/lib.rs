struct Solution ;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut cache: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
        Solution::dp(s.chars().collect::<Vec<char>>().as_slice(), &mut cache)
    }

    pub fn dp(digits: &[char], cache: &mut std::collections::HashMap<String, i32>) -> i32 {
        if digits.len() == 0 {
            return 1;
        }

        let label = digits.iter().collect::<String>();
        match cache.get(&label) {
            Some(result) => {
                return *result;
            }
            None => ()
        };

        let dp1_result = if digits[0] == '0' { 0 } else { Solution::dp(&digits[1..], cache)};
        if digits.len() == 1 {
            return dp1_result;
        }

        let dp2_result = if digits[0] == '1' || (digits[0] == '2' && digits[1] < '7') {
            Solution::dp(&digits[2..], cache)
        } else {
            0
        };

        let result = dp1_result + dp2_result;
        cache.insert(label, result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_decodings() {
        let s = "0".to_string();
        let result = 0;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "1".to_string();
        let result = 1;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "2".to_string();
        let result = 1;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "01".to_string();
        let result = 0;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "10".to_string();
        let result = 1;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "11".to_string();
        let result = 2;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "27".to_string();
        let result = 1;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "26".to_string();
        let result = 2;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "12".to_string();
        let result = 2;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "226".to_string();
        let result = 3;
        assert_eq!(Solution::num_decodings(s), result);

        let s = "06".to_string();
        let result = 0;
        assert_eq!(Solution::num_decodings(s), result);

        /*
        let s = "111111111111111111111111111111111111111111111".to_string();
        let result = 0;
        assert_eq!(Solution::num_decodings(s), result);
        */
    }
}

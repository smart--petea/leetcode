struct Solution;
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut rev_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        rev_vec.reverse();

        let mut result: String = String::new();
        let mut ki: i32 = 0;
        let mut c_upper: char = 'a';
        for c in rev_vec {
            if c == '-' {
                continue;
            }
            ki = ki + 1;

            c_upper = Solution::upper(c);
            result.insert(0, c_upper);

            if ki == k {
                result.insert(0, '-');
                ki = 0;
            }
        }

        if ki == 0 && result.len() > 0 {
            return result[1..].to_string();
        } else {
            return result;
        }
    }

    pub fn upper(c: char) -> char {
        if c >= 'a' {
            return (c as u8 - 32u8) as char;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn license_key_formatting() {
        /*
        assert_eq!(Solution::license_key_formatting("2e-9-w".to_string(), 4), "2E9W".to_string());
        assert_eq!(Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4), "5F3Z-2E9W".to_string());
        assert_eq!(Solution::license_key_formatting("2-5g-3-J".to_string(), 2), "2-5G-3J".to_string());
        assert_eq!(Solution::license_key_formatting("--a-a-a-a--".to_string(), 2), "AA-AA".to_string());
        */
        assert_eq!(Solution::license_key_formatting("---".to_string(), 3), "".to_string());
    }
}

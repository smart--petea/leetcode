struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();

        let mut i = 0;
        while i < s.len() && s[i] == ' ' {
            i = i + 1;
        }

        let mut  sign = 1i32;
        if i < s.len() && (s[i] == '-' || s[i] == '+') {
            if s[i] == '-' {
                sign = -1 * sign;
            }

            i = i + 1;
        }

        let mut base = 0i32;
        while i < s.len() && s[i] >= '0' && s[i] <= '9' {
            if base > i32::MAX/10  || (base == i32::MAX/10 && s[i] > '7') {
                if sign == 1 {
                    return i32::MAX;
                } else {
                    return i32::MIN;
                }
            }
            base = 10 * base + s[i] as i32 - '0' as i32;
            i = i + 1;
        }

        base * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_atoi() {
        let s = "42".to_string();
        let result = 42;
        assert_eq!(Solution::my_atoi(s), result);

        let s = "-42".to_string();
        let result = -42;
        assert_eq!(Solution::my_atoi(s), result);

        let s = "4193 with words".to_string();
        let result = 4193;
        assert_eq!(Solution::my_atoi(s), result);

        let s = "words and 987".to_string();
        let result = 0;
        assert_eq!(Solution::my_atoi(s), result);

        let s ="-91283472332".to_string();
        let result = -2147483648;
        assert_eq!(Solution::my_atoi(s), result);

        let s ="+-12".to_string();
        let result = 0;
        assert_eq!(Solution::my_atoi(s), result);
    }
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut precedent: i32 = i32::MAX;

        for chr in s.chars() {
            let mut digit: i32 = match chr {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("wrong char {}", chr),
            };

            if precedent < digit {
                digit = digit - 2 * precedent;
            }
            precedent = digit;

            sum = sum + digit;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}

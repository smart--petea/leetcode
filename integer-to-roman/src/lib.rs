use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn int_to_roman(num: i32) -> String {
        let mut map  = HashMap::new();
        map.insert(1000, "M");
        map.insert(900, "CM");
        map.insert(500, "D");
        map.insert(400, "CD");
        map.insert(100, "C");
        map.insert(90, "XC");
        map.insert(50, "L");
        map.insert(40, "XL");
        map.insert(10, "X");
        map.insert(9, "IX");
        map.insert(5, "V");
        map.insert(4, "IV");
        map.insert(1, "I");

        let mut num_str: String = String::new();
        let mut num_m: i32 = num;
        for key in vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1] {
            let q = Solution::quotient(num_m, key);
            if q == 0 {
                continue;
            }

            let roman_combination = map.get(&key).unwrap();
            num_str.push_str(&roman_combination.repeat(q as usize));

            num_m = Solution::remainder(num_m, key);
            if num_m == 0 {
                break;
            }
        }

        num_str
    }

    pub fn quotient(a: i32, b: i32) -> i32 {
        let q = a as f32 / b as f32;
        q.floor() as i32
    }

    pub fn remainder(a: i32, b:i32) -> i32 {
        let q = Solution::quotient(a, b);

        a - b * q
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn int_to_roman() {
        assert_eq!(Solution::int_to_roman(1), "I".to_string());
        assert_eq!(Solution::int_to_roman(5), "V".to_string());
        assert_eq!(Solution::int_to_roman(1000), "M".to_string());
        assert_eq!(Solution::int_to_roman(900), "CM".to_string());
        assert_eq!(Solution::int_to_roman(500), "D".to_string());
        assert_eq!(Solution::int_to_roman(400), "CD".to_string());
        assert_eq!(Solution::int_to_roman(100), "C".to_string());
        assert_eq!(Solution::int_to_roman(90), "XC".to_string());
        assert_eq!(Solution::int_to_roman(50), "L".to_string());
        assert_eq!(Solution::int_to_roman(40), "XL".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(10), "X".to_string());

        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(27), "XXVII".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn quotient() {
        assert_eq!(Solution::quotient(3,2), 1);
        assert_eq!(Solution::quotient(5,2), 2);
    }

    #[test]
    fn remainder() {
        assert_eq!(Solution::remainder(3,2), 1);
        assert_eq!(Solution::remainder(5,2), 1);
    }
}

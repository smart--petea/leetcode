struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column: f64 = column_number as f64;
        let mut column_str: String = String::new();

        loop {
            if column == 0_f64 {
                break;
            }

            let (quote, rest) = Solution::div_26(column as f64);
            column = quote;

            let c: char = std::char::from_u32((rest as u32) + 64).unwrap();
            column_str.insert(0, c);
        }

        column_str
    }

    pub fn div_26(column: f64) -> (f64, u32) {
        let quote: f64 = (column / 26_f64).floor();
        let rest: f64 = column - quote * 26_f64;

        if rest == 0_f64 {
            (quote - 1_f64, 26)
        } else {
            (quote, rest as u32)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW".to_string());
    }

    #[test]
    fn div_26() {
        assert_eq!(Solution::div_26(1_f64), (0_f64, 1));
        assert_eq!(Solution::div_26(25_f64), (0_f64, 25));
        assert_eq!(Solution::div_26(26_f64), (0_f64, 26));
        assert_eq!(Solution::div_26(27_f64), (1_f64, 1));
    }
}

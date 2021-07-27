struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut column: i32 = 0;

        for c in column_title.chars() {
            column = column * 26;
            let x = (c as i32) - 64;
            column = column + x;
        }

        column
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn title_to_number() {
        /*
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
        */
        assert_eq!(Solution::title_to_number("FXSHRXW".to_string()), 2147483647);
    }
}

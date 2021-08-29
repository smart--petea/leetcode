struct Solution;
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let index: usize = 0;
        let mut width: i32 = 0;
        let mut lines: i32 = 1;
        let mut char_width: i32 = 0; 
        for c in s.chars() {
            char_width = widths[c as usize - 97usize];

            if width + char_width > 100 {
                lines = lines + 1;
                width = 0;
            }

            width = width + char_width;
        }

        vec![lines, width]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn number_of_lines() {
        let widths: Vec<i32> = vec!(10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10);
        let s: String = "abcdefghijklmnopqrstuvwxyz".to_string();
        let result: Vec<i32> = vec!(3, 60);
        assert_eq!(Solution::number_of_lines(widths, s), result);

        let widths: Vec<i32> = vec!(4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10);
        let s: String = "bbbcccdddaaa".to_string();
        let result: Vec<i32> = vec!(2, 4);
        assert_eq!(Solution::number_of_lines(widths, s), result);
    }
}

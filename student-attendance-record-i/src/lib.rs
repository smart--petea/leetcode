struct Solution;
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut abbsences: usize = 0; 
        let mut lates: usize = 0;

        for c in s.chars() {
            if c == 'A' {
                abbsences = abbsences + 1;

                if abbsences >= 2usize {
                    return false;
                }
            }

            if c != 'L' {
                lates = 0;
            } else {
                lates = lates + 1;

                if lates >= 3usize {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn check_record() {
        assert_eq!(Solution::check_record("PPALLP".to_string()), true);
        assert_eq!(Solution::check_record("PPALLL".to_string()), false);
    }
}

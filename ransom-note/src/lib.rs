struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut ransom_statistic: [usize; 26] = [0; 26];
        let mut magazine_statistic: [usize; 26] = [0; 26];

        let mut i: usize = 0;

        for c in ransom_note.chars() {
            i = c as usize - 97;
            ransom_statistic[i] = ransom_statistic[i] + 1;
        }

        for c in magazine.chars() {
            i = c as usize - 97;
            magazine_statistic[i] = magazine_statistic[i] + 1 ;
        }

        for i in 0..26 {
            if ransom_statistic[i] > magazine_statistic[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn can_construct() {
        assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
        assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
        assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
    }
}

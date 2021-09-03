struct Solution;
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut large_groups: Vec<Vec<i32>> = Vec::new();

        let mut precedent: char = '1';
        let mut start: usize = 0;
        let mut len: usize = 0;
        for (i, c) in s.chars().enumerate() {
            if c == precedent  {
                continue;
            }

            if i - start >= 3 {
                large_groups.push(vec!(start as i32, (i - 1) as i32));
            }

            precedent = c; 
            start = i;
        }

        if s.len() > 0 && s.chars().last().unwrap() == precedent && s.len() - start >= 3 {
            large_groups.push(vec!(start as i32, (s.len() - 1) as i32));
        };

        large_groups
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn large_group_positions() {
        let s: String = "abbxxxxzzy".to_string();
        let result: Vec<Vec<i32>> = vec!(vec!(3, 6));
        assert_eq!(Solution::large_group_positions(s), result);

        let s: String = "abc".to_string();
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::large_group_positions(s), result);

        let s: String = "abcdddeeeeaabbbcd".to_string();
        let result: Vec<Vec<i32>> = vec!(vec!(3, 5), vec!(6,9), vec!(12,14));

        let s: String = "aaa".to_string();
        let result: Vec<Vec<i32>> = vec!(vec!(0, 2));
        assert_eq!(Solution::large_group_positions(s), result);
    }
}

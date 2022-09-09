struct Solution;
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut i = 0;
        let mut result = vec![' '; indices.len()];
        for c in s.chars() {
            result[indices[i] as usize] = c;

            i = i + 1;
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "abc".to_string();
        let indices = vec![0,1,2];
        let output = Solution::restore_string(s, indices);
        assert_eq!(output, "abc".to_string());

        let s = "codeleet".to_string();
        let indices = vec![4,5,6,7,0,2,1,3];
        let output = Solution::restore_string(s, indices);
        assert_eq!(output, "leetcode".to_string());
    }
}

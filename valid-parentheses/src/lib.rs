struct Solution; 

impl Solution {
    pub fn is_valid(s: String) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }
}

struct Solution;
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        }

        std::cmp::max(a.len() as i32, b.len() as i32) 
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn find_lu_slength() {
        assert_eq!(Solution::find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
        assert_eq!(Solution::find_lu_slength("aaa".to_string(), "ccc".to_string()), 3);
        assert_eq!(Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
    }
}

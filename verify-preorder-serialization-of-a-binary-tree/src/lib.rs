struct Solution;
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut diff = 1;
        for c in preorder.split(',') {
            diff = diff - 1;
            if diff < 0 {
                return false;
            }

            if c != "#" {
                diff = diff + 2;
            }
        }

        diff == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_serialization() {
        /*
        let preorder = "#".to_string();
        let result = true;
        assert_eq!(Solution::is_valid_serialization(preorder), result);

        let preorder = "9".to_string();
        let result = false;
        assert_eq!(Solution::is_valid_serialization(preorder), result);

        let preorder = "9,#,#".to_string();
        let result = true;
        assert_eq!(Solution::is_valid_serialization(preorder), result);

        let preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#".to_string();
        let result = true;
        assert_eq!(Solution::is_valid_serialization(preorder), result);
        */
        let preorder = "9,#,92,#,#".to_string();
        let result = true;
        assert_eq!(Solution::is_valid_serialization(preorder), result);
    }
}

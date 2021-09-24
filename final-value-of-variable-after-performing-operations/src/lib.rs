struct Solution;
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0i32;

        for operation in operations {
            result = result +  match operation.as_str() {
                "++X" | "X++" => {
                    1
                }
                _ => -1,
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_value_after_operations() {
        let operations = vec!["--X".to_string(),"X++".to_string(),"X++".to_string()];
        let result = 1;
        assert_eq!(Solution::final_value_after_operations(operations), result);

        let operations = vec!["++X".to_string(),"X++".to_string(),"X++".to_string()];
        let result = 3;
        assert_eq!(Solution::final_value_after_operations(operations), result);

        let operations = vec!["++X".to_string(),"X++".to_string(),"X--".to_string(), "--X".to_string()];
        let result = 0;
        assert_eq!(Solution::final_value_after_operations(operations), result);
    }
}

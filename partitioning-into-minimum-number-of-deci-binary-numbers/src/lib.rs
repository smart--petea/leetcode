struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut result = '0';

        for c in n.chars() {
            if c > result {
                if c == '9' {
                    return 9;
                }

                result = c;
            }
        }

        result as i32 - '0' as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_partitions() {
        let n = "32".to_string();
        let result = 3;
        assert_eq!(Solution::min_partitions(n), result);

        let n = "82734".to_string();
        let result = 8;
        assert_eq!(Solution::min_partitions(n), result);

        let n = "27346209830709182346".to_string();
        let result = 9;
        assert_eq!(Solution::min_partitions(n), result);
    }
}

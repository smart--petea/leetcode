struct Solution;
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut chars_to_be_left = vec![true; s.len()];
        let mut count_precedent = 0;
        for i in 0..s.len() {
            if count_precedent == 0 {
                chars_to_be_left[i] = false;
            } else if count_precedent == 1 && s[i] == ')' {
                chars_to_be_left[i] = false;
            }

            if s[i] == '(' {
                count_precedent = count_precedent + 1;
            } else {
                count_precedent = count_precedent - 1;
            }
        }

        let mut result = String::new();
        for i in 0..s.len() {
            if chars_to_be_left[i] == false {
                continue;
            }

            result.push(s[i]);
        }

        result
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_outer_parentheses() {
        let s = "()".to_string();
        let result = "".to_string();
        assert_eq!(Solution::remove_outer_parentheses(s), result);

        let s = "(())".to_string();
        let result = "()".to_string();
        assert_eq!(Solution::remove_outer_parentheses(s), result);

        let s = "(()())(())".to_string();
        let result = "()()()".to_string();
        assert_eq!(Solution::remove_outer_parentheses(s), result);

        let s = "(()())(())(()(()))".to_string();
        let result = "()()()()(())".to_string();
        assert_eq!(Solution::remove_outer_parentheses(s), result);

        let s = "()()".to_string();
        let result = "".to_string();
        assert_eq!(Solution::remove_outer_parentheses(s), result);
    }
}

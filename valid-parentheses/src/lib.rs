struct Solution; 

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if Solution::is_open_bracket(&c) {
                stack.push(c);
                continue;
            }

            if stack.is_empty() {
                return false;
            }

            let open_bracket = Solution::get_open_bracket(&c);
            if open_bracket != stack.pop() {
                return false;
            }
        }

        stack.is_empty()
    }

    fn is_open_bracket(c: &char) -> bool {
         *c == '(' || *c == '{' || *c == '['
    }

    fn get_open_bracket(close_bracket: &char) -> Option<char> {
        match close_bracket {
            ')' => Some('('),
            '}' => Some('{'),
            ']' => Some('['),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);
    }
}

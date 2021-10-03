struct Solution;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();

        Solution::backtrack(&mut list, "".to_string(), 0, 0, n as usize);

        list
    }

    pub fn backtrack(list: &mut Vec<String>, mut strr: String, open: usize, close: usize, max: usize) {
        if strr.len() == max * 2 {
            list.push(strr);
            return;
        }

        if open < max {
            let mut str1 = strr.clone();
            str1.push('(');
            Solution::backtrack(list, str1, open + 1, close, max);
        }

        if close < open {
            strr.push(')');
            Solution::backtrack(list, strr, open, close + 1, max);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis() {
        let n = 3i32;
        let result = vec!["((()))".to_string(),"(()())".to_string(),"(())()".to_string(),"()(())".to_string(),"()()()".to_string()];
        assert_eq!(Solution::generate_parenthesis(n), result);

        let n = 1i32;
        let result = vec!["()".to_string()];
        assert_eq!(Solution::generate_parenthesis(n), result);
    }
}

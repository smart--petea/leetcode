struct Solution; 
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        
        for c in s.chars() {
            match stack.pop() {
                Some(d) if c == d => (),
                Some(d) => {
                    stack.push(d);
                    stack.push(c);
                }
                None => {
                    stack.push(c);
                }
            }
        }

        stack.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates() {
        let s = "azxxzy".to_string();
        let result = "ay".to_string();
        assert_eq!(Solution::remove_duplicates(s), result);

        let s = "abbaca".to_string();
        let result = "ca".to_string();
        assert_eq!(Solution::remove_duplicates(s), result);
    }
}

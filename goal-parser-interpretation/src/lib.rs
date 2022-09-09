struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        let mut iter = command.bytes().into_iter();

        while let Some(c) = iter.next() {
            match c {
               b'G' => result.push('G'),
               _ => {
                   match iter.next() {
                       Some(b')') => result.push('o'),
                       _ => {
                           result.push_str("al");
                           iter.next();
                           iter.next();
                       }
                   }
               }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let command = "G()(al)".to_string();
        let output = Solution::interpret(command);
        assert_eq!(output, "Goal".to_string());

        let command = "G()()()(al)".to_string();
        let output = Solution::interpret(command);
        assert_eq!(output, "Goooal".to_string());
    }
}

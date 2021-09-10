struct Solution;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
       let mut letter_logs: Vec<[String; 2]> = Vec::new(); 
       let mut digits_logs: Vec<String> = Vec::new(); 

       let mut index: usize;
       let mut identifier: String = String::new();
       let mut tail: String = String::new();
       let mut c: char;
       for log in logs {
           index = (*log).find(' ').unwrap();
           tail = log[(index+1)..].to_string();
           c = tail.chars().nth(0).unwrap();
           if c >= '0' && c <= '9' {
               digits_logs.push(log.to_string());
               continue;
           }

           identifier = log[0..index].to_string();
           letter_logs.push([identifier, tail]);
       }

       if letter_logs.len() > 0 {
           letter_logs.sort_by(|left, right| left[0].cmp(&right[0]));
           letter_logs.sort_by(|left, right| left[1].cmp(&right[1]));
       }

       let mut result: Vec<String> = Vec::new();
       for mut letter_log in letter_logs {
           letter_log[0].push(' ');
           letter_log[0].push_str(&letter_log[1].to_string());
           result.push(letter_log[0].to_string());
       }

       for digit_log in digits_logs.iter() {
           result.push(digit_log.to_string());
       }

       result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorder_log_files() {
        let logs: Vec<String> = vec!(
            "dig1 8 1 5 1".to_string(),
            "let1 art can".to_string(),
            "dig2 3 6".to_string(),
            "let2 own kit dig".to_string(),
            "let3 art zero".to_string()
        );
        let result: Vec<String> = vec!(
            "let1 art can".to_string(),
            "let3 art zero".to_string(),
            "let2 own kit dig".to_string(),
            "dig1 8 1 5 1".to_string(),
            "dig2 3 6".to_string(),
        );
        assert_eq!(Solution::reorder_log_files(logs), result);

        let logs: Vec<String> = vec!(
            "a1 9 2 3 1".to_string(),
            "g1 act car".to_string(),
            "zo4 4 7".to_string(),
            "ab1 off key dog".to_string(),
            "a8 act zoo".to_string()
        );

        let result: Vec<String> = vec!(
            "g1 act car".to_string(),
            "a8 act zoo".to_string(),
            "ab1 off key dog".to_string(),
            "a1 9 2 3 1".to_string(),
            "zo4 4 7".to_string()
        );
        assert_eq!(Solution::reorder_log_files(logs), result);
    }
}

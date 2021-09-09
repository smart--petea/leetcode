struct Solution;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
       let mut unique_emails: std::collections::HashSet<String> = std::collections::HashSet::new();

       let mut at_position: usize;
       let mut local_name: String = String::new();
       let mut domain_name: String = String::new();
       let mut unique_email: String = String::new();
       for email in emails {
           at_position = email.find('@').unwrap();
           local_name = email[..at_position].to_string();
           domain_name = email[at_position..].to_string();

           match local_name.find('+') {
               Some(plus_position) => {
                   local_name = local_name[..plus_position].to_string();
               }
               None => ()
           }

           unique_email = local_name.replace(".", "");
           unique_email.push_str(&domain_name);

           unique_emails.insert(unique_email);
       }

       unique_emails.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_unique_emails() {
        let emails: Vec<String> = vec!(
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        );
        let result: i32 = 2;
        assert_eq!(Solution::num_unique_emails(emails), result);

        let emails: Vec<String> = vec!(
            "a@leetcode.com".to_string(),
            "b@leetcode.com".to_string(),
            "c@leetcode.com".to_string()
        );
        let result: i32 = 3;
        assert_eq!(Solution::num_unique_emails(emails), result);
    }
}

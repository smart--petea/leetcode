struct Solution;
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
       let mut allowed_ar = [false; 26]; 
       const B_A: u8 = b'a';

       for b in allowed.bytes() {
            allowed_ar[(b - B_A) as usize] = true;
       }

       let mut count = 0i32;
       'outer: for word in words {
           for b in word.bytes() {
               if allowed_ar[(b - B_A) as usize] == false {
                   continue 'outer;
               }
           }

           count += 1;
       }

       count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
         let allowed = "ab".to_string(); 
         let words = vec!["ad".to_string(),"bd".to_string(),"aaab".to_string(),"baa".to_string(),"badab".to_string()];
         let expected = 2;
         let output = Solution::count_consistent_strings(allowed, words);
         assert_eq!(expected, output);

         let allowed = "abc".to_string(); 
         let words = vec!["a".to_string(),"b".to_string(),"c".to_string(),"ab".to_string(),"ac".to_string(),"bc".to_string(),"abc".to_string()];
         let expected = 7;
         let output = Solution::count_consistent_strings(allowed, words);
         assert_eq!(expected, output);

         let allowed = "cad".to_string(); 
         let words = vec!["cc".to_string(), "acd".to_string(), "b".to_string(),"ba".to_string(), "bac".to_string(), "bad".to_string(),"ac".to_string(),"d".to_string()];
         let expected = 4;
         let output = Solution::count_consistent_strings(allowed, words);
         assert_eq!(expected, output);


    }
}

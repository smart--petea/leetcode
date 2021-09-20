struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
       let l1 = str1.len(); 
       let l2 = str2.len();
       let l = std::cmp::min(l1, l2);

       if l == 0 {
           return String::new();
       }


       let str1_vec = str1.chars().collect::<Vec<char>>();
       let str2_vec = str2.chars().collect::<Vec<char>>();

       let mut result = String::new();
       let mut temp = String::new();
       let mut tl: usize;

       for i in 0..l {
           if str1_vec[i] != str2_vec[i] {
               break;
           }

           temp.push(str1_vec[i]);
           tl = temp.len();
           if l1 % tl == 0 && l2 % tl == 0 && temp.repeat(l1/tl) == str1 && temp.repeat(l2/tl) == str2 {
               result = temp.clone();
           }
       }

       result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_strings() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABCABCABC".to_string();
        let result = "ABC".to_string();
        assert_eq!(Solution::gcd_of_strings(str1, str2), result);

        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let result = "".to_string();
        assert_eq!(Solution::gcd_of_strings(str1, str2), result);
    }
}

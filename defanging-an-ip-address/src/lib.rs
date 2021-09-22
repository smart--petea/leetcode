struct Solution;
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
       let mut result = String::new();
       let rep = "[.]";

       for c in address.chars() {
           if c == '.' {
               result.push_str(rep);
           } else {
               result.push(c);
           }
       }

       result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defang_i_paddr() {
        let address = "1.1.1.1".to_string();
        let result = "1[.]1[.]1[.]1".to_string();
        assert_eq!(Solution::defang_i_paddr(address), result);

        let address = "255.100.50.0".to_string();
        let result = "255[.]100[.]50[.]0".to_string();
        assert_eq!(Solution::defang_i_paddr(address), result);
    }
}

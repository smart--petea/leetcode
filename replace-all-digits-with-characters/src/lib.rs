struct Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result: Vec<u8> = Vec::with_capacity(s.len());

        let mut s_iter = s.bytes().into_iter();
        
        loop {
            let c = s_iter.next();
            if c.is_none() {
                break;
            }
            let c = c.unwrap();
            result.push(c);

            let i = s_iter.next();
            if i.is_none() {
                break;
            }
            let i = i.unwrap() - b'0';
            result.push(c + i);

        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let  s = "a1c1e1".to_string();
        let output = Solution::replace_digits(s);
        let expected = "abcdef".to_string();
        assert_eq!(output, expected);

        let  s = "a1b2c3d4e".to_string();
        let output = Solution::replace_digits(s);
        let expected = "abbdcfdhe".to_string();
        assert_eq!(output, expected);
    }
}

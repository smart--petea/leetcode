struct Solution;
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let bytes = s.bytes().collect::<Vec<u8>>();
        let mut result: Vec<String> = Vec::new();

        let start_letter_byte = bytes[0];
        let start_index = bytes[1];
        let end_letter_byte = bytes[3];
        let end_index = bytes[4];

        for letter_byte in start_letter_byte..=end_letter_byte {
            let letter = letter_byte as char;
            for index in start_index..=end_index {
                let mut result_str = String::new();
                result_str.push(letter);
                result_str.push(index as char);
                result.push(result_str);
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
        let s = "K1:L2".to_string();
        let expected = vec!["K1", "K2", "L1", "L2"];
        let output = Solution::cells_in_range(s);
        assert_eq!(expected, output);


        let s = "A1:F1".to_string();
        let expected = vec!["A1", "B1", "C1", "D1", "E1", "F1"];
        let output = Solution::cells_in_range(s);
        assert_eq!(expected, output);
    }
}

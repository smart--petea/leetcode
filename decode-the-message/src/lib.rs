struct Solution;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = ['-'; 26];    

        let mut current_char = 0u8;
        for key_char in key.chars() {
            if key_char == ' ' {
                continue;
            }

            let position = key_char as usize - 97;
            if map[position] != '-' {
                continue;
            }

            map[position] = (current_char + 97) as char;
            current_char = current_char + 1;
        }

        let mut result = String::new();
        for message_char in message.chars() {
            if message_char == ' ' {
                result.push(' ');
                continue;
            }

            let position = message_char as usize - 97;
            result.push(map[position]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        let expected = "the five boxing wizards jump quickly".to_string();
        let output = Solution::decode_message(key, message);
        assert_eq!(expected, output);

        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let expected = "this is a secret".to_string();
        let output = Solution::decode_message(key, message);
        assert_eq!(expected, output);
    }
}

struct Solution;

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let state = s
            .bytes()
            .into_iter()
            .fold((false, 0), |mut state: (bool, i32), v: u8| -> (bool, i32) {
                match v {
                    b'*' if state.0 == false => {
                        state.1 += 1;
                    }
                    b'|' => {
                        state.0 = !state.0;
                    }
                    _ => {}
                }

                state
            });

        state.1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "l|*e*et|c**o|*de|".to_string();
        let expected = 2;
        let output = Solution::count_asterisks(input);
        assert_eq!(expected, output);

        let input = "iamprogrammer".to_string();
        let expected = 0;
        let output = Solution::count_asterisks(input);
        assert_eq!(expected, output);

        let input = "yo|uar|e**|b|e***au|tifu|l".to_string();
        let expected = 5;
        let output = Solution::count_asterisks(input);
        assert_eq!(expected, output);
    }
}

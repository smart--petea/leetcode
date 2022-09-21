struct Solution;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s
            .bytes()
            .into_iter()
            .fold((0, 0), |mut acc, byte| -> (i32, i32) {
                match byte {
                    b'(' => {
                        acc.1 += 1;
                    }
                    b')' => {
                        acc.1 -= 1;
                    }
                    _ => {
                        return acc;
                    }
                };

                acc.0 = std::cmp::max(acc.0, acc.1.abs());

                acc
            })        
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input =  "(1+(2*3)+((8)/4))+1".to_string();
        let expected = 3;
        let output = Solution::max_depth(input);
        assert_eq!(output, expected);


        let input =  "(1)+((2))+(((3)))".to_string();
        let expected = 3;
        let output = Solution::max_depth(input);
        assert_eq!(output, expected);
    }
}

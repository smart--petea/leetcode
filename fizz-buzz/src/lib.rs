struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::with_capacity(n as usize);

        for i in 1..(n + 1) {
            result.insert((i - 1) as usize, i.to_string());
        }

        for i in (3..(n + 1)).step_by(3) {
            result[(i - 1) as usize] = "Fizz".to_string();
        }

        for i in (5..(n + 1)).step_by(5) {
            result[(i - 1) as usize] = "Buzz".to_string();
        }

        for i in (15..(n + 1)).step_by(15) {
            result[(i - 1) as usize] = "FizzBuzz".to_string();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn fizz_buzz() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1".to_string(), "2".to_string(), "Fizz".to_string()]);
        assert_eq!(Solution::fizz_buzz(5), vec!["1".to_string(),"2".to_string(),"Fizz".to_string(),"4".to_string(),"Buzz".to_string()]);
        assert_eq!(Solution::fizz_buzz(15), vec!["1".to_string(),"2".to_string(),"Fizz".to_string(),"4".to_string(),"Buzz".to_string(),"Fizz".to_string(),"7".to_string(),"8".to_string(),"Fizz".to_string(),"Buzz".to_string(),"11".to_string(),"Fizz".to_string(),"13".to_string(),"14".to_string(),"FizzBuzz".to_string()]);
    }
}

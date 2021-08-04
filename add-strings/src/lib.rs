struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut min_vec: Vec<char>;
        let mut max_vec: Vec<char>;

        if num1.len() < num2.len() {
            min_vec = num1.chars().collect::<Vec<char>>();
            max_vec = num2.chars().collect::<Vec<char>>();
        } else {
            min_vec = num2.chars().collect::<Vec<char>>();
            max_vec = num1.chars().collect::<Vec<char>>();
        }

        let mut d: u8 = 0;
        let mut a: u8 = 0;
        let mut b: u8 = 0;
        let mut sum: u8 = 0;
        let min_vec_len: usize = min_vec.len() as usize;
        let max_vec_len: usize = max_vec.len() as usize;
        let mut result: Vec<char> = vec!['0'; max_vec.len()];
        let zero: u8 = '0' as u8;
        for i in 0..max_vec_len {
            a = max_vec[max_vec_len - i - 1] as u8 - zero;
            b = if i < min_vec_len {
                min_vec[min_vec_len - i -1] as u8 - zero
            } else {
                0
            };

            sum = a + b + d;

            if sum < 10 {
                result[max_vec_len - i - 1] = (sum + zero) as char;
                d = 0;
            } else {
                result[max_vec_len - i - 1] = (sum + zero - 10) as char;
                d = 1;
            }
        }

        if d == 0 {
            result.into_iter().collect::<String>()
        } else {
            let mut one: String = "1".to_string();
            one.push_str(result.into_iter().collect::<String>().as_str());
            one
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn add_strings() {
        assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134".to_string());
        assert_eq!(Solution::add_strings("77".to_string(), "456".to_string()), "533".to_string());
        assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0".to_string());
    }
}

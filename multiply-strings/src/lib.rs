struct Solution;
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.chars().collect::<Vec<char>>();    
        let num2 = num2.chars().collect::<Vec<char>>();

        let m = num1.len();
        let n = num2.len();
        let mut pos = vec![0u8; m + n]; 
        
        let mut d1: u8;
        let mut d2: u8;
        let mut mul: u8;
        let mut sum: u8;
        let mut p1: usize;
        let mut p2: usize;

        let zerou8 = '0' as u8;
        for i in 0..m {
            for j in 0..n {
                d1 = num1[m - i - 1] as u8 - zerou8;
                d2 = num2[n - j - 1] as u8 - zerou8;
                mul = d1 * d2;
                p2 = m - i + n - j - 1;
                p1 = p2 - 1;
                sum = mul + pos[p2];

                pos[p1] = pos[p1] + sum / 10;
                pos[p2] = sum % 10;
            }
        }

        let mut i = 0usize;
        while i < pos.len() && pos[i] == 0 {
            i = i + 1;
        }

        if i >= pos.len() {
            return "0".to_string();
        }

        let mut result = String::new();
        while i < pos.len() {
            result.push((pos[i] + zerou8) as char);
            i = i + 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply() {
        let num1 = "2".to_string();
        let num2 = "3".to_string();
        let result = "6".to_string();
        assert_eq!(Solution::multiply(num1, num2), result);

        let num1 = "123".to_string();
        let num2 = "456".to_string();
        let result = "56088".to_string();
        assert_eq!(Solution::multiply(num1, num2), result);

        let num1 = "123".to_string();
        let num2 = "0".to_string();
        let result = "0".to_string();
        assert_eq!(Solution::multiply(num1, num2), result);
    }
}

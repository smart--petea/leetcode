struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        let mut vecs = vec![Vec::new(); num_rows];
        let s_vec = s.chars().collect::<Vec<char>>();
        let mut i = 0usize;
        'outer: while i < s.len() {
            for idx in 0..(num_rows as usize) {
                vecs[idx].push(s_vec[i]);
                i = i + 1;
                if i >= s.len() {
                    break 'outer;
                }
            }

            if num_rows > 1 {
                for idx in 1..(num_rows-1) as usize  {
                    vecs[num_rows - idx - 1].push(s_vec[i]);
                    i = i + 1;
                    if i >= s.len() {
                        break 'outer;
                    }

                }
            }
        }

        let mut result = vecs[0].iter().collect::<String>();
        for i in 1..num_rows as usize {
            result.push_str(&vecs[i].iter().collect::<String>());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let result = "PAHNAPLSIIGYIR".to_string();
        assert_eq!(Solution::convert(s, num_rows), result);

        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let result = "PINALSIGYAHRPI".to_string();
        assert_eq!(Solution::convert(s, num_rows), result);

        let s = "A".to_string();
        let num_rows = 1;
        let result = "A".to_string();
        assert_eq!(Solution::convert(s, num_rows), result);
    }
}

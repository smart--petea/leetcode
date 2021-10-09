struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s_len = s.len();

        let mut A: i32;
        let mut B: i32;
        let mut C: i32;
        let mut D: i32;

        let mut i: usize;
        let mut j: usize;

        let mut result = Vec::new();
        let mut result_s = String::new();

        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    for d in 1..=3 {
                        if a+b+c+d != s_len {
                            continue;
                        }

                        i = 0;
                        j = a;
                        A = s[i..j].parse::<i32>().unwrap();
                        if A > 255 {
                            continue;
                        }

                        i = j;
                        j = j + b;
                        B = s[i..j].parse::<i32>().unwrap();
                        if B > 255 {
                            continue;
                        }

                        i = j;
                        j = j + c;
                        C = s[i..j].parse::<i32>().unwrap();
                        if C > 255 {
                            continue;
                        }

                        i = j;
                        j = j + d;
                        D = s[i..j].parse::<i32>().unwrap();
                        if D > 255 {
                            continue;
                        }


                        result_s = format!("{}.{}.{}.{}", A, B, C, D);
                        if result_s.len() == s_len + 3 {
                            result.push(result_s);
                        }
                    }
                }
            }
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_ip_addresses() {
        let s = "25525511135".to_string();
        let result = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];

        let s = "0000".to_string();
        let result = vec!["0.0.0.0".to_string()];
        assert_eq!(Solution::restore_ip_addresses(s), result);

        let s = "010010".to_string();
        let result = vec!["0.10.0.10".to_string(), "0.100.1.0".to_string()];
        assert_eq!(Solution::restore_ip_addresses(s), result);

        let s = "101023".to_string();
        let result = vec!["1.0.10.23".to_string(),"1.0.102.3".to_string(),"10.1.0.23".to_string(),"10.10.2.3".to_string(),"101.0.2.3".to_string()];
        assert_eq!(Solution::restore_ip_addresses(s), result);
    }
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut j: i32 = s.len() as i32;
        let mut i: i32 = -1;

        loop {
            i = i + 1;
            j = j - 1;
            if i >= j  {
                break;
            }

            loop {
                let c = s_vec[i as usize];
                if Solution::is_char(&c) || Solution::is_digit(&c) {
                    break;
                }

                i = i + 1;
                if i > j {
                    return true;
                }
            }

            loop {
                let c = s_vec[j as usize];
                if Solution::is_char(&c) || Solution::is_digit(&c) {
                    break;
                }

                j = j - 1;
                if j == i {
                    break;
                }
            }

            let chi = s_vec[i as usize];
            let chj = s_vec[j as usize];
            if chj == chi {
                continue
            }

            if Solution::is_char(&chi) && Solution::is_char(&chj) &&  (chj as i32- chi as i32).abs() == 32 {
                continue
            }

            return false;
        }

        true
    }

    pub fn is_char(c: &char) -> bool {
        let chr = *c;

        (chr  >= 'a' && chr <= 'z') || (chr  >= 'A' && chr <= 'Z')
    }

    pub fn is_digit(d: &char) -> bool {
        let chr = *d;

        chr >= '0' && chr <= '9'
    }

    pub fn is_printable(c: &char) -> bool {
        Solution::is_char(c) || Solution::is_digit(c)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome("0P".to_string()), false);
        assert_eq!(Solution::is_palindrome(".,".to_string()), true);
        assert_eq!(Solution::is_palindrome("aba".to_string()), true);
        assert_eq!(Solution::is_palindrome("amanaplanacanalpanama".to_string()), true);
        assert_eq!(Solution::is_palindrome("raceacar".to_string()), false);
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("A A".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }
}

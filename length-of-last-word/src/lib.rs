struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let svec = s.chars().collect::<Vec<char>>();
        let l = s.len();
        let mut t = l;

        for i in 0..l {
            if svec[l - i - 1] != ' ' {
                break;
            }
            t = l - i - 1;
        }

        for i in 0..t {
            if svec[t - i - 1] == ' ' {
                return i as i32;
            }
        }

        t as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word(" ".to_string()), 0);
        assert_eq!(Solution::length_of_last_word("a ".to_string()), 1);
    }
}

//idea from www.youtube.com/watch?v=tUxW1JwEb9M

struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
        let t_vec: Vec<char> = t.chars().collect::<Vec<char>>();

        let mut si: i32 = (s_vec.len() - 1) as i32;
        let mut ti: i32 = (t_vec.len() - 1) as i32;
        let mut backspaces: i32 = 0;

        while si >= 0 || ti >= 0 {
            backspaces = 0;
            while si >= 0 && (backspaces > 0 || s_vec[si as usize] == '#') {
                if s_vec[si as usize] == '#' {
                    backspaces = backspaces + 1;
                } else {
                    backspaces = backspaces - 1;
                }

                si = si - 1;
            }

            backspaces = 0;
            while ti >= 0 && (backspaces > 0 || t_vec[ti as usize] == '#') {
                if t_vec[ti as usize] == '#' {
                    backspaces = backspaces + 1;
                } else {
                    backspaces = backspaces - 1;
                }

                ti = ti - 1;
            }

            if si >= 0 && ti >= 0 {
                if s_vec[si as usize] != t_vec[ti as usize] {
                    return false;
                }
                si = si - 1;
                ti = ti - 1;
            } else {
                if si >= 0 || ti >= 0 {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn backspace_compare() {
        let s: String = "ab#c".to_string();
        let t: String = "ad#c".to_string();
        let result: bool = true;
        assert_eq!(Solution::backspace_compare(s, t), result);

        let s: String = "ab##".to_string();
        let t: String = "c#d#".to_string();
        let result: bool = true;
        assert_eq!(Solution::backspace_compare(s, t), result);

        let s: String = "a##c".to_string();
        let t: String = "#a#c".to_string();
        let result: bool = true;
        assert_eq!(Solution::backspace_compare(s, t), result);

        let s: String = "a#c".to_string();
        let t: String = "b".to_string();
        let result: bool = false;
        assert_eq!(Solution::backspace_compare(s, t), result);

        let s: String = "bbbextm".to_string();
        let t: String = "bbb#extm".to_string();
        let result: bool = false;
        assert_eq!(Solution::backspace_compare(s, t), result);
    }
}

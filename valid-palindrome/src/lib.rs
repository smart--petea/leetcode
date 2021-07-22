struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();

        let mut j: i32 = s.len() as i32;
        let mut i: i32 = -1;

        loop {
            println!("i = {}", i);
            let (chi, i1) = Solution::next_char(&s_vec, i, j);
            i = i1;
            println!("i = {} c={}", i, s_vec[i as usize]);
            if chi == None {
                return false;
            }

            println!("j = {}", j);
            let (chj, j1) = Solution::precedent_char(&s_vec, i, j);
            j = j1;
            println!("j={} c={}", j, s_vec[j as usize]);
            if chj == None {
                return false;
            }

            if Solution::is_the_same(chi, chj) == false {
                return false;
            }

            if i == j - 1  || i == j {
                break;
            }

        }

        true
    }

    pub fn next_char(v: &Vec<char>, i: i32, j: i32) -> (Option<char>, i32) {
        for j in (i + 1)..(v.len() as i32) {
            if v[j as usize] == ' ' {
                continue;
            }

            return (Some(v[j as usize]), j);
        }

        (None, 0)
    }

    pub fn precedent_char(v: &Vec<char>, i: i32, j: i32) -> (Option<char>, i32) {
        for j in 0..i {
            let k = (i - j - 1) as usize;
            if v[k] == ' ' {
                continue;
            }

            return (Some(v[k]), k as i32);
        }

        (None, 0)
    }

    pub fn is_the_same(o1: Option<char>, o2: Option<char>) -> bool {
        let c1 = o1.unwrap() as i32;
        let c2 = o2.unwrap() as i32;

        if c1 == c2 {
            return true;
        }

        (c1 - c2).abs() == 32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_palindrome() {
        /*
        assert_eq!(Solution::is_palindrome("1".to_string()), true);
        assert_eq!(Solution::is_palindrome("11".to_string()), true);
        assert_eq!(Solution::is_palindrome("aba".to_string()), true);
        assert_eq!(Solution::is_palindrome("amanaplanacanalpanama".to_string()), true);
        assert_eq!(Solution::is_palindrome("raceacar".to_string()), false);
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        */
        assert_eq!(Solution::is_palindrome("A A".to_string()), true);
    }

    #[test]
    fn next_char() {
        let v = vec!['a', 'b', ' ', 'c', ' ', 'd'];
        assert_eq!(Solution::next_char(&v, -1), (Some('a'), 0));
        assert_eq!(Solution::next_char(&v, 0), (Some('b'), 1));
        assert_eq!(Solution::next_char(&v, 1), (Some('c'), 3));
        assert_eq!(Solution::next_char(&v, 3), (Some('d'), 5));
        assert_eq!(Solution::next_char(&v, 5), (None, 0));
    }

    #[test]
    fn precedent_char() {
        let v = vec!['a', 'b', ' ', 'c', ' ', 'd'];
        assert_eq!(Solution::precedent_char(&v, v.len() as i32), (Some('d'), (v.len() - 1) as i32));
        assert_eq!(Solution::precedent_char(&v, 5), (Some('c'), 3));
        assert_eq!(Solution::precedent_char(&v, 3), (Some('b'), 1));
        assert_eq!(Solution::precedent_char(&v, 1), (Some('a'), 0));
        assert_eq!(Solution::precedent_char(&v, 0), (None, 0));
    }

    #[test]
    fn is_the_same() {
        assert_eq!(Solution::is_the_same(Some('a'), Some('a')), true);
        assert_eq!(Solution::is_the_same(Some('a'), Some('b')), false);
        assert_eq!(Solution::is_the_same(Some('a'), Some('A')), true);
    }
}

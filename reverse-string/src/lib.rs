struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i: usize = 0;
        let mut j: usize = s.len() - 1;
        let mut c: char = 'a';

        while i < j {
            c = s[i];
            s[i] = s[j];
            s[j] = c;

            i = i + 1;
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn reverse_string() {
        let mut s: Vec<char> = vec!['h','e','l','l','o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o','l','l','e','h']);

        let mut s: Vec<char> = vec!['H','a','n','n','a','h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h','a','n','n','a','H']);
    }
}

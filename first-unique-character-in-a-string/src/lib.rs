use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::<char, i32>::new();

        for char_indice in s.char_indices() {
            let chr = char_indice.1;

            match map.get(&chr) {
                None => {
                    map.insert(chr, char_indice.0 as i32);
                }
                Some(cnt) => {
                    map.insert(chr, -1);
                }
            }
        }

        for chr in s.chars() {
            match map.get(&chr) {
                Some(index) if *index != -1 => {
                    return *index;
                }
                _ => (),
            };
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}

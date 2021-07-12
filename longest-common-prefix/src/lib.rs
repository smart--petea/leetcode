struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest: String = strs[0].clone();
        for s in strs.iter() {
            if s.len() < longest.len() {
                longest = s.clone();
            }
        }

        if longest.len() == 0 {
            return String::new();
        }

        for s in strs.iter() {
            longest = Solution::get_prefix(&s, &longest);
        }

        longest
    }

    pub fn get_prefix(first: &String, second: &String) -> String {
        let mut prefix: String = String::new();
        let pairs = first.chars().zip(second.chars());

        for pair in pairs {
            if pair.0 == pair.1 {
                prefix.push(pair.0)
            } else {
                return prefix;
            }
        }

        return prefix;

    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
    }

    #[test]
    fn test_get_prefix() {
        let first: String = "abc123".to_string();
        let second: String = "abc321".to_string();
        assert_eq!(Solution::get_prefix(&first, &second), "abc".to_string());
    }
}

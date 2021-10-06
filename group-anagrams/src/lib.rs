struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return Vec::new();
        }

        let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut hash = String::new();
        let mut hash_vec: Vec<char> = Vec::new();
        for strr in strs {
            hash_vec = strr.chars().collect::<Vec<char>>();
            hash_vec.sort_by(|a, b| b.cmp(a));
            hash = hash_vec.iter().collect::<String>();

            match map.get_mut(&hash) {
                Some(v) => {
                    v.push(strr);
                }
                None => {
                    map.insert(hash, vec![strr]);
                }
            }
        }

        let mut result = Vec::new();
        for (s, v) in map {
            result.push(v)
        }

        result 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_anagrams() {
        /*
        let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
        let result = vec![vec!["eat".to_string(), "tea".to_string(), "ate".to_string()], vec!["bat".to_string()],  vec!["tan".to_string(), "nat".to_string()]];
        assert_eq!(Solution::group_anagrams(strs), result);
        */

        let strs = vec!["a".to_string()];
        let result = vec![vec!["a".to_string()]];
        assert_eq!(Solution::group_anagrams(strs), result);

        let strs = vec!["".to_string()];
        let result = vec![vec!["".to_string()]];
        assert_eq!(Solution::group_anagrams(strs), result);
    }
}

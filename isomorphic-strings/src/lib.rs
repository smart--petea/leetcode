use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let svec: Vec<char> = s.chars().collect::<Vec<char>>();
        let tvec: Vec<char> = t.chars().collect::<Vec<char>>();

        let mut map_st: HashMap::<char, char> = HashMap::<char, char>::new();
        let mut map_ts: HashMap::<char, char> = HashMap::<char, char>::new();

        for i in 0..svec.len() {
            let cs = svec[i];

            match map_st.get(&cs) {
                Some(ct) => {
                    if *ct != tvec[i] {
                        return false;
                    }
                },
                None => {
                    let ct: char = tvec[i];
                    match map_ts.get(&ct) {
                        Some(cs1) => {
                            return false;
                        }
                        None => {
                            map_st.insert(cs, ct);
                            map_ts.insert(ct, cs);
                        }
                    }
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
    fn is_isomorphic() {
        assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
        assert_eq!(Solution::is_isomorphic("foo".to_string(), "bar".to_string()), false);
        assert_eq!(Solution::is_isomorphic("paper".to_string(), "title".to_string()), true);
        assert_eq!(Solution::is_isomorphic("badc".to_string(), "baba".to_string()), false);
    }
}

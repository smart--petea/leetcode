struct Solution;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words_counter: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for w in s1.split(' ') {
            match words_counter.get(w) {
                Some(i) if *i == 1usize => {
                    words_counter.insert(w.to_string(), 2);
                }
                Some(i) => (),
                None => {
                    words_counter.insert(w.to_string(), 1);
                }
            }
        }

        for w in s2.split(' ') {
            match words_counter.get(w) {
                Some(i) if *i == 1usize => {
                    words_counter.insert(w.to_string(), 2);
                }
                Some(i) => (),
                None => {
                    words_counter.insert(w.to_string(), 1);
                }
            }
        }

        let mut result: Vec<String> = Vec::new();
        for (w, cnt) in words_counter {
            if cnt == 2 {
                continue;
            }

            result.push(w);
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn uncommon_from_sentences() {
        let s1: String = "this apple is sweet".to_string();
        let s2: String = "this apple is sour".to_string();
        let result: Vec<String> = vec!("sweet".to_string(), "sour".to_string());
        assert_eq!(Solution::uncommon_from_sentences(s1, s2), result);
    }
}

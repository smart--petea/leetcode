struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_index: usize = match &rule_key as &str {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!("wrong rule_key={}", rule_key)
        };

        items.iter().fold(0, |mut acc: i32, v: &Vec<String>| -> i32 {
            if v[rule_index] == rule_value {
                acc += 1;
            }

            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let items = vec![
            vec!["phone".to_string(),"blue".to_string(),"pixel".to_string()],
            vec!["computer".to_string(),"silver".to_string(),"phone".to_string()],
            vec!["phone".to_string(),"gold".to_string(),"iphone".to_string()]];
        let rule_key = "type".to_string();
        let rule_value = "phone".to_string();

        let output = Solution::count_matches(items, rule_key, rule_value);
        let expected = 2;
        assert_eq!(output, expected);

        let items = vec![
            vec!["phone".to_string(),"blue".to_string(),"pixel".to_string()],
            vec!["computer".to_string(),"silver".to_string(),"phone".to_string()],
            vec!["phone".to_string(),"gold".to_string(),"iphone".to_string()]];
        let rule_key = "color".to_string();
        let rule_value = "silver".to_string();

        let output = Solution::count_matches(items, rule_key, rule_value);
        let expected = 1;
        assert_eq!(output, expected);
    }
}

struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if name.len() == typed.len() && name == typed {
            return true;
        }

        if name.len() > typed.len() {
            return false;
        }

        let name_vec: Vec<char> = name.chars().collect::<Vec<char>>();
        let typed_vec: Vec<char> = typed.chars().collect::<Vec<char>>();
        let mut i: usize = 0;

        for j in 0..typed_vec.len() {
            if i < name_vec.len() && name_vec[i] == typed_vec[j] {
                i = i + 1;
            } else if j == 0 || typed_vec[j] != typed_vec[j - 1] {
                return false;
            }
        }

        i == name_vec.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn is_long_pressed_name() {
        let name: String = "alex".to_string();
        let typed: String = "aaleex".to_string();
        let result: bool = true; 
        assert_eq!(Solution::is_long_pressed_name(name, typed), result);

        let name: String = "saeed".to_string();
        let typed: String = "ssaaed".to_string();
        let result: bool = false; 
        assert_eq!(Solution::is_long_pressed_name(name, typed), result);

        let name: String = "leelee".to_string();
        let typed: String = "lleelee".to_string();
        let result: bool = true; 
        assert_eq!(Solution::is_long_pressed_name(name, typed), result);

        let name: String = "laiden".to_string();
        let typed: String = "laiden".to_string();
        let result: bool = true; 
        assert_eq!(Solution::is_long_pressed_name(name, typed), result);
    }
}

struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
       if s.len() != goal.len() {
           return false;
       }

       if s == goal {
           let mut counts: [usize; 26] = [0; 26];
           let mut index: usize = 0;
           for c in s.chars() {
               index = c as usize - 97usize;
               if counts[index] == 1 {
                   return true;
               }

               counts[index] = 1;
           }

           return false;
       }

       let s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
       let goal_vec: Vec<char> = goal.chars().collect::<Vec<char>>();
       let mut diffs: Vec<(char, char)> = Vec::new();

       for i in 0..s_vec.len() {
           if s_vec[i] == goal_vec[i] {
               continue;
           }

           if diffs.len() == 2 {
               return false;
           }

           diffs.push((s_vec[i], goal_vec[i]));
       }

       if diffs.len() == 1 {
           return false;
       }

       diffs[0].0 == diffs[1].1 && diffs[0].1 == diffs[1].0
    }
}
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn buddy_strings() {
        /*
        let s: String = "a".to_string();
        let goal: String = "aa".to_string();
        let result: bool = false;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "ab".to_string();
        let goal: String = "ab".to_string();
        let result: bool = false;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "aba".to_string();
        let goal: String = "aba".to_string();
        let result: bool = true;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "aba".to_string();
        let goal: String = "aba".to_string();
        let result: bool = true;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "salut".to_string();
        let goal: String = "satul".to_string();
        let result: bool = true;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "salutcb".to_string();
        let goal: String = "satulbc".to_string();
        let result: bool = false;
        assert_eq!(Solution::buddy_strings(s, goal), result);

        let s: String = "salu".to_string();
        let goal: String = "satu".to_string();
        let result: bool = false;
        assert_eq!(Solution::buddy_strings(s, goal), result);
        */

        let s: String = "salu".to_string();
        let goal: String = "satu".to_string();
        let result: bool = false;
        assert_eq!(Solution::buddy_strings(s, goal), result);
    }
}

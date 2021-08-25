struct Solution;
impl Solution {

    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut results: Vec<String> = Vec::with_capacity(words.len());

        for word in words {
            results.push(Solution::to_morse(word))
        }

        results.sort();
        results.dedup();

        results.len() as i32
    }

    pub fn to_morse(s: String) -> String {
        let morse_codes: [&str; 26] = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]; 
        let mut result: String = String::new();

        let mut index: usize = 0;
        for c in s.chars() {
            index = c as usize - 97usize;
            result.push_str(morse_codes[index]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn unique_morse_representations() {
        let words: Vec<String> = vec!["gin".to_string(),"zen".to_string(),"gig".to_string(),"msg".to_string()];
        assert_eq!(Solution::unique_morse_representations(words), 2);

        let words: Vec<String> = vec!["a".to_string()];
        assert_eq!(Solution::unique_morse_representations(words), 1);
    }
}

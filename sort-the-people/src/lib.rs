struct Solution;
impl Solution {
    pub fn sort_people(mut names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        let len = heights.len();
        if len == 1 {
            return names;
        }

        let mut swaps = true;

        while swaps {
            swaps = false;
            for i in 0..(len-1) {
                if heights[i] < heights[i+1] {
                    heights.swap(i, i+1);
                    names.swap(i, i+1);
                    swaps = true;
                }
            }
        }

        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        let output = Solution::sort_people(names, heights);
        let expected = vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()];
        assert_eq!(output, expected);

        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155, 185, 150];
        let output = Solution::sort_people(names.clone(), heights);
        let expected = vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()];
        assert_eq!(output, expected);

        let names = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()];
        let heights = vec![155, 185, 150, 200];
        let output = Solution::sort_people(names.clone(), heights);
        let expected = vec!["D".to_string(), "B".to_string(), "A".to_string(), "C".to_string()];
        assert_eq!(output, expected);
    }
}

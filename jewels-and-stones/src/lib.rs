struct Solution;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut count: i32 = 0;
        for s in stones.chars() {
            if jewels.find(s) != None {
                count = count + 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn num_jewels_in_stones() {
        assert_eq!(Solution::num_jewels_in_stones("aA".to_string(), "aAABbbbb".to_string()), 3);
        assert_eq!(Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}

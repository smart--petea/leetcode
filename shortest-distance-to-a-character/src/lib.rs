struct Solution;
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut c_indexes: Vec<i32> = Vec::new();
        c_indexes.push(-(s.len() as i32));
        for (index, cc) in s.char_indices() {
            if cc == c {
                c_indexes.push(index as i32)
            }
        }
        c_indexes.push((s.len() + s.len()) as i32);

        let mut current_index: usize = 0;
        let mut result: Vec<i32> = Vec::with_capacity(s.len());

        let mut first: i32 = 0;
        let mut second: i32 = 0;

        for (index, cc) in s.char_indices() {
            if cc == c {
                current_index = current_index + 1;
                result.push(0i32);
                continue;
            }

            first = c_indexes[current_index];
            second = c_indexes[current_index + 1];

            result.push(std::cmp::min(index as i32 - first, second - index as i32));
        }


        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn shortest_to_char() {
        let c: char = 'e';
        let s: String = "loveleetcode".to_string();
        let result: Vec<i32> = vec!(3,2,1,0,1,0,0,1,2,2,1,0);
        assert_eq!(Solution::shortest_to_char(s, c), result);

        let c: char = 'b';
        let s: String = "aaab".to_string();
        let result: Vec<i32> = vec!(3,2,1,0);
        assert_eq!(Solution::shortest_to_char(s, c), result);
    }
}

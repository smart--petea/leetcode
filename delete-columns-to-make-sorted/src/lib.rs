struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs: usize = strs_vec.len() - 1;

        let strs_vec: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let columns_len: usize = strs_vec[0].len();
        let mut to_be_deleted: Vec<bool> = vec![false; columns_len];
        
        for column in 0..columns_len {
            if to_be_deleted[column] {
                continue;
            }

            for row in 0..strs_len {
                if strs_vec[row][column] > strs_vec[row+1][column] {
                    to_be_deleted[column] = true;
                }
            }
        }

        let mut count: i32 = 0;
        for i in 0..columns_len {
            if to_be_deleted[i] {
                count = count + 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_deletion_size() {
        let strs: Vec<String> = vec!("cba".to_string(),"daf".to_string(),"ghi".to_string());
        let result: i32 = 1;
        assert_eq!(Solution::min_deletion_size(strs), result);

        let strs: Vec<String> = vec!("a".to_string(), "b".to_string());
        let result: i32 = 0;
        assert_eq!(Solution::min_deletion_size(strs), result);

        let strs: Vec<String> = vec!("zyx".to_string(),"wvu".to_string(),"tsr".to_string());
        let result: i32 = 3;
        assert_eq!(Solution::min_deletion_size(strs), result);

        let strs: Vec<String> = vec!("rrjk".to_string(),"furt".to_string(),"guzm".to_string());
        let result: i32 = 2;
        assert_eq!(Solution::min_deletion_size(strs), result);
    }
}

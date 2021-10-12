struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        if s1.is_empty() && s2 == s3 {
            return true;
        }

        if s2.is_empty() && s1 == s3 {
            return true;
        }

        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let s3 = s3.chars().collect::<Vec<char>>();

        let mut result = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i==0 && j==0 {
                    result[i][j] = true;
                } else if i==0 {
                    result[i][j] = result[i][j-1] && s2[j-1] == s3[i+j-1];
                } else if j== 0 {
                    result[i][j] = result[i-1][j] && s1[i-1] == s3[i+j-1];
                } else {
                    result[i][j] = (result[i-1][j] && s1[i-1] == s3[i+j-1]) || (result[i][j-1] && s2[j-1] == s3[i+j-1]);
                }
            }
        }

        result[s1.len()][s2.len()]
    }
}

/*
#[derive(Debug)]
struct MyPoint {
    x_s1: usize,
    y_s2: usize,
}

struct Solution;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let s3 = s3.chars().collect::<Vec<char>>();

        let s1_len = s1.len();
        let s2_len = s2.len();
        let s3_len = s3.len();

        let mut visited = vec![vec![false; s2_len+1]; s1_len+1];
        let mut queue = vec![MyPoint{x_s1: 0, y_s2: 0}]; 

        let mut i: usize;

        loop {
            match queue.pop() {
                Some(myPoint) => {
                    if myPoint.x_s1 + myPoint.y_s2 == s3_len {
                        return true;
                    }

                    i = myPoint.x_s1 + myPoint.y_s2;

                    if myPoint.x_s1 < s1_len {
                        if visited[myPoint.x_s1][myPoint.y_s2] == false && s3[i] == s1[myPoint.x_s1] {
                            visited[myPoint.x_s1][myPoint.y_s2] == true;
                            queue.push(MyPoint{x_s1: myPoint.x_s1 + 1, y_s2: myPoint.y_s2});
                        }
                    }

                    if myPoint.y_s2 < s2_len {
                        if visited[myPoint.x_s1][myPoint.y_s2] == false && s3[i] == s2[myPoint.y_s2] {
                            visited[myPoint.x_s1][myPoint.y_s2] == true;
                            queue.push(MyPoint{x_s1: myPoint.x_s1, y_s2: myPoint.y_s2 + 1});
                        }
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_interleave() {
        let s1 = "ab".to_string();
        let s2 = "cd".to_string();
        let s3 = "acbd".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "ab".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "ba".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "bb".to_string();
        let result = false;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        let result = false;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "def".to_string();
        let s2 = "".to_string();
        let s3 = "def".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        let s1 = "ccabcaacaacccaabacbcacac".to_string();
        let s2 = "ccbcbacbaaccabbabbca".to_string();
        let s3 = "cccbccbabcacaacabacaaccccaaabbabbabccabcacac".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);

        /*
        let s1 = "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        let s2 = "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string();
        let s3 = "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        let result = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), result);
        */
    }
}

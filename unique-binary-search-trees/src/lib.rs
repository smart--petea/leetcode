struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }

        let n = n as usize;
        let mut g = vec![0i32; n+1];
        g[0] = 1;
        g[1] = 1;

        for i in 2..=n  {
            for j in 1..=i {
                g[i] += g[j-1] * g[i-j];
            }
        }

        g[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_trees() {
        let n = 0;
        let result = 1;
        assert_eq!(Solution::num_trees(n), result);

        let n = 1;
        let result = 1;
        assert_eq!(Solution::num_trees(n), result);

        let n = 2;
        let result = 2;
        assert_eq!(Solution::num_trees(n), result);

        let n = 3;
        let result = 5;
        assert_eq!(Solution::num_trees(n), result);
    }
}

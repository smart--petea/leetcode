struct Solution;
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut rights: usize = 0;
        let mut lefts: usize = 0;
        let mut ups: usize = 0;
        let mut downs: usize = 0;

        for c in moves.chars() {
            match c {
                'R' => rights = rights + 1,
                'L' => lefts = lefts + 1,
                'U' => ups = ups + 1,
                'D' => downs = downs + 1,
                _ => ()
            }
        }

        rights == lefts && ups == downs
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn judge_circle() {
        assert_eq!(Solution::judge_circle("UD".to_string()), true);
        assert_eq!(Solution::judge_circle("LL".to_string()), false);
        assert_eq!(Solution::judge_circle("RRDD".to_string()), false);
        assert_eq!(Solution::judge_circle("LDRRLRUULR".to_string()), false);
    }
}

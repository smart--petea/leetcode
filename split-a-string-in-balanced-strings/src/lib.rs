struct Solution;
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let evidence = s.chars()
            .into_iter()
            .fold(Evidence::default(), |mut evidence: Evidence, ch: char| -> Evidence {
                match ch {
                    'R' => evidence.add_r(),
                    _ => evidence.add_l()
                }

                evidence
            });    

        evidence.substrings as i32
    }
}

#[derive(Default)]
struct Evidence {
    rs: usize,
    ls: usize,
    substrings: usize,
}

impl Evidence {
    fn add_r(&mut self) {
        self.rs += 1;

        self.update_substrings();
    }

    fn add_l(&mut self) {
        self.ls += 1;

        self.update_substrings();
    }

    fn update_substrings(&mut self) {
        if self.rs != self.ls {
            return;
        }

        self.substrings += 1;
        self.rs = 0;
        self.ls = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "RLRRLLRLRL".to_string();
        let result = Solution::balanced_string_split(input);
        let expected = 4;
        assert_eq!(expected, result);

        let input = "RLRRRLLRLL".to_string();
        let result = Solution::balanced_string_split(input);
        let expected = 2;
        assert_eq!(expected, result);

        let input = "LLLLRRRR".to_string();
        let result = Solution::balanced_string_split(input);
        let expected = 1;
        assert_eq!(expected, result);
    }
}

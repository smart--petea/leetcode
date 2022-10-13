struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut slots = [0_u8; 10];
        let mut count = 0;

        let mut rings_iter = rings.bytes().into_iter();
        loop {
            let next = rings_iter.next();
            if next.is_none() {
                break;
            }

            let color = next.unwrap();
            let position = (rings_iter.next().unwrap() - b'0') as usize;
            if slots[position] == 7 {
                continue;
            }

            match color {
                b'R' => {
                    slots[position] |= 1;
                }
                b'G' => {
                    slots[position] |= 2;
                }
                _ => {
                    slots[position] |= 4;
                }
            }

            if slots[position] == 7 {
                count += 1;

                if count == 10 {
                    break;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rings = "B0B6G0R6R0R6G9".to_string();
        let output = Solution::count_points(rings);
        let expected = 1;
        assert_eq!(output, expected);

        let rings = "B0R0G0R9R0B0G0".to_string();
        let output = Solution::count_points(rings);
        let expected = 1;
        assert_eq!(output, expected);

        let rings = "G4".to_string();
        let output = Solution::count_points(rings);
        let expected = 0;
        assert_eq!(output, expected);
    }
}

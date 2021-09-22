struct Solution;
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let year = date[..4].parse::<i32>().unwrap();
        let month = date[5..7].parse::<i32>().unwrap();
        let day = date[8..].parse::<i32>().unwrap();

        let months = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut count_days = 0i32;

        for i in 0..(month - 1) {
            count_days += months[i as usize];
        }

        if month>2 && year%4==0 && year>1990 {
            count_days += 1;
        }

        count_days + day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_of_year() {
        let date = "2019-01-09".to_string();
        let result = 9;
        assert_eq!(Solution::day_of_year(date), result);

        let date = "2019-02-10".to_string();
        let result = 41;
        assert_eq!(Solution::day_of_year(date), result);

        let date = "2003-03-01".to_string();
        let result = 60;
        assert_eq!(Solution::day_of_year(date), result);

        let date = "2004-03-01".to_string();
        let result = 61;
        assert_eq!(Solution::day_of_year(date), result);
    }
}

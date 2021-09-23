struct Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let (date1, date2) = if date1.cmp(&date2) == std::cmp::Ordering::Greater {
            (date2, date1)
        } else {
            (date1, date2)
        };


        let d1 = date1[8..].parse::<i32>().unwrap();
        let d2 = date2[8..].parse::<i32>().unwrap();

        let m1 = date1[5..7].parse::<i32>().unwrap();
        let m2 = date2[5..7].parse::<i32>().unwrap();

        let y1 = date1[..4].parse::<i32>().unwrap();
        let y2 = date2[..4].parse::<i32>().unwrap();

        Solution::num_days(d2, m2, y2) - Solution::num_days(d1, m1, y1)
    }

    pub fn num_days(d: i32, m: i32, y: i32) -> i32 {
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut sum: i32 = d;

        sum = sum + (y-1) * 365;
        sum = sum + (((y-1)/400) + ((y-1)/4) - ((y-1)/100));

        for i in 0..(m-1) {
            sum = sum + months[i as usize];
        }

        if m > 2 && (y%4==0 && y%100!=0 || y%400==0) {
            sum = sum + 1
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_between_dates() {
        let date1 = "2019-06-29".to_string();
        let date2 = "2019-06-30".to_string();
        let result = 1;
        assert_eq!(Solution::days_between_dates(date1, date2), result);

        let date1 = "2020-01-15".to_string();
        let date2 = "2019-12-31".to_string();
        let result = 15;
        assert_eq!(Solution::days_between_dates(date1, date2), result);
    }
}

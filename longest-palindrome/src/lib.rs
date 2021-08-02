struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut small_counts: [i32; 26] = [0; 26];
        let mut big_counts: [i32; 26] = [0; 26];

        let mut i: usize = 0;
        for c in s.chars() {
            if c <= 'Z' {
                i = c as usize - 65usize;
                big_counts[i] = big_counts[i] + 1;
            } else {
                i = c as usize - 97usize;
                small_counts[i] = small_counts[i] + 1;
            }
        }

        /*
        for i in 0..26 {
            println!("{} - {}", (i as u8 + 97_u8) as char, small_counts[i]);
        }

        for i in 0..26 {
            println!("{} - {}", (i as u8 + 65_u8) as char, big_counts[i]);
        }
        */

        let mut sum: i32 = 0;
        let mut has_even: i32 = 0;
        for i in 0..26 {
            if (small_counts[i] & 1) == 1 {
                sum = sum + small_counts[i] - 1;
                has_even = 1;
            } else {
                sum = sum + small_counts[i];
            }
        }

        for i in 0..26 {
            if (big_counts[i] & 1) == 1{
                sum = sum + big_counts[i] - 1;
                has_even = 1;
            } else {
                sum = sum + big_counts[i];
            }
        }

        sum + has_even
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
        assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
        assert_eq!(Solution::longest_palindrome("bab".to_string()), 3);
        assert_eq!(Solution::longest_palindrome("cbab".to_string()), 3);
        assert_eq!(Solution::longest_palindrome("bB".to_string()), 1);
        assert_eq!(Solution::longest_palindrome("bBb".to_string()), 3);

        let long: String = "civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_string();
        assert_eq!(Solution::longest_palindrome(long), 983);
    }
}

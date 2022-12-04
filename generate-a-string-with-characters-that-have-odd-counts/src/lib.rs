struct Solution;
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n == 0 {
            return String::new();
        }

        if n % 2 == 1 {
            return "t".repeat(n as usize);
        } else {
            let mut t = "t".repeat((n - 1) as usize);
            t.push('s');

            return t;
        }
    }
}

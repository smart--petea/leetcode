struct Solution;
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        let two_dots = "..";
        let one_dot = ".";
        for component in path.split("/") {
            if component == two_dots && stack.is_empty() == false {
                stack.pop();
            } else if !(component.is_empty() || component == one_dot || component == two_dots) {
                stack.push(component);
            }
        }

        let mut result = "/".to_string();
        result.push_str(&stack.join("/"));

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_path() {
        let path = "/home/".to_string();
        let result = "/home".to_string();
        assert_eq!(Solution::simplify_path(path), result);

        let path = "/../".to_string();
        let result = "/".to_string();
        assert_eq!(Solution::simplify_path(path), result);

        let path = "/home//foo".to_string();
        let result = "/home/foo".to_string();
        assert_eq!(Solution::simplify_path(path), result);
    }
}

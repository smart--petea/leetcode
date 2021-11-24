#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Codec {

}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut sb = String::new();
        Self::serialize_inner(&root, &mut sb);

        sb
    }

    fn serialize_inner(root: &Option<Rc<RefCell<TreeNode>>>, sb: &mut String) {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            sb.push_str(&root_node.val.to_string());
            sb.push_str(",");

            Self::serialize_inner(&root_node.left, sb);
            Self::serialize_inner(&root_node.right, sb);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let mut q = data.split(",").collect::<Vec<&str>>();
        q.reverse();

        Self::deserialize_inner(&mut q, std::i32::MIN, std::i32::MAX)
    }

    fn deserialize_inner(q: &mut Vec<&str>, lower: i32, upper: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(s) = q.last() {
            if s.is_empty() {
                return None;
            }

            let val = s.parse::<i32>().unwrap();
            if val < lower || val > upper {
                return None;
            }
            q.pop();

            let left = Self::deserialize_inner(q, lower, val);
            let right = Self::deserialize_inner(q, val, upper);

            return Some(Rc::new(RefCell::new(TreeNode{val: val, left: left, right: right})));
        }

        None
    }
}

/**
 * Your Codec object will be instantiated
 and called as such:
 * let obj = Codec::new();
 * let data: String =
 obj.serialize(strs);
 * let ans:
 Option<Rc<RefCell<TreeNode>>> =
 obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n4})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n3})));
        let obj = Codec::new();
        let data: String = obj.serialize(n2.clone());
        let result = obj.deserialize(data);
        assert_eq!(n2, result);
    }
}

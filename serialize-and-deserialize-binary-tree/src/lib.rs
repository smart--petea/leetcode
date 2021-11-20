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
        let mut string_builder: String = String::new();
        Self::build_string(&root, &mut string_builder);
        string_builder
    }

    fn build_string(root: &Option<Rc<RefCell<TreeNode>>>, string_builder: &mut String) {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            string_builder.push_str(&root_node.val.to_string());
            string_builder.push_str(",");

            Self::build_string(&root_node.left, string_builder);
            Self::build_string(&root_node.right, string_builder);
        } else {
            string_builder.push_str("X,");
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = data.split(',').collect::<Vec<&str>>();
        nodes.reverse();
        Self::build_tree(&mut nodes)
    }

    fn build_tree(nodes: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = nodes.pop() {
            if val == "X" {
                return None;
            } else {
                let val_i32 = val.parse::<i32>().unwrap();
                let mut node = TreeNode::new(val_i32);
                node.left = Self::build_tree(nodes);
                node.right = Self::build_tree(nodes);

                return Some(Rc::new(RefCell::new(node)));
            }
        }

        None
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n4, right: n5})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let obj = Codec::new();
        let data = obj.serialize(n1.clone());
        let r1 = obj.deserialize(data);
        assert_eq!(n1, r1);
    }
}

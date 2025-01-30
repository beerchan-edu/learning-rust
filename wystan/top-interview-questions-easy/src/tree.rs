use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// Definition for a binary tree node.
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
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            1 + max(max_depth(node.left.clone()), max_depth(node.right.clone()))
        }
        None => 0,
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_with_range(root).0
}

fn is_valid_bst_with_range(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
    match root {
        None => (true, i32::MAX, i32::MIN),
        Some(node) => {
            let node_ref = node.borrow();
            let (left_is_bst, left_min, left_max) = is_valid_bst_with_range(node_ref.left.clone());
            let (right_is_bst, right_min, right_max) =
                is_valid_bst_with_range(node_ref.right.clone());

            let is_bst = left_is_bst
                && right_is_bst
                && (node_ref.left.is_none() || left_max < node_ref.val)
                && (node_ref.right.is_none() || right_min > node_ref.val);

            // Compute min and max values of the subtree
            let min_val = if node_ref.left.is_some() {
                left_min
            } else {
                node_ref.val
            };
            let max_val = if node_ref.right.is_some() {
                right_max
            } else {
                node_ref.val
            };

            (is_bst, min_val, max_val)
        }
    }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return root == rotate(root.clone());
}

pub fn rotate(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => return None,
        Some(node) => {
            let node_ptr = node.borrow();
            let rotated = TreeNode {
                val: node_ptr.val,
                left: rotate(node_ptr.right.clone()),
                right: rotate(node_ptr.left.clone()),
            };
            return Some(Rc::new(RefCell::new(rotated)));
        }
    }
}
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    level_order_for_list(vec![root])
}

pub fn level_order_for_list(nodes: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Vec<Vec<i32>> {
    let mut values: Vec<i32> = Vec::new();
    let mut next_level: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    for node in nodes {
        match node {
            None => continue,
            Some(node) => {
                let node_ptr = node.borrow();
                values.push(node_ptr.val);
                next_level.push(node_ptr.left.clone());
                next_level.push(node_ptr.right.clone());
            }
        }
    }
    if values.is_empty() {
        return vec![];
    }
    let mut result = vec![values];
    if next_level.is_empty() {
        return result;
    }
    let next_level_answer = level_order_for_list(next_level);
    if next_level_answer.is_empty() {
        return result;
    }
    result.extend(next_level_answer);
    return result;
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    match nums.as_slice() {
        [] => None,
        [a] => Some(Rc::new(RefCell::new(TreeNode::new(*a)))),
        [a, b] => Some(Rc::new(RefCell::new(TreeNode {
            val: *a,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(*b)))),
        }))),
        [a, b, c] => Some(Rc::new(RefCell::new(TreeNode {
            val: *b,
            left: Some(Rc::new(RefCell::new(TreeNode::new(*a)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(*c)))),
        }))),
        _ => {
            let mid_index = nums.len().checked_div(2).expect("division failes");
            return Some(Rc::new(RefCell::new(TreeNode {
                val: nums.get(mid_index).copied().unwrap_or(0),
                left: sorted_array_to_bst(nums[..mid_index].to_vec()),
                right: sorted_array_to_bst(nums[mid_index + 1..].to_vec()),
            })));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_depth(None);
        assert_eq!(result, 0);
    }

    #[test]
    fn one_node_depth() {
        let result = max_depth(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))));
        assert_eq!(result, 2);
    }
}

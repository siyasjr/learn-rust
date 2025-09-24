// Problem: Return the bottom-up level order traversal of a binary tree’s nodes.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let mut res = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut level = vec![];
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node_b = node.borrow();
                level.push(node_b.val);
                if let Some(l) = &node_b.left { queue.push_back(l.clone()); }
                if let Some(r) = &node_b.right { queue.push_back(r.clone()); }
            }
            res.push(level);
        }
        res.reverse();
        res
    }
}

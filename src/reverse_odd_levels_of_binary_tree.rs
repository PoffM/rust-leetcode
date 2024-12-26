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

    pub fn from_vec(mut vec: Vec<i32>) -> Rc<RefCell<TreeNode>> {
        vec.reverse();
        if let Some(val) = vec.pop() {
            let root = Rc::new(RefCell::new(TreeNode::new(val)));
            let mut nodes = vec![root.clone()];

            while !vec.is_empty() {
                let mut new_nodes = vec![];
                for node in nodes.iter() {
                    if let Some(val) = vec.pop() {
                        let left = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().left = Some(left.clone());
                        new_nodes.push(left);
                    }
                    if let Some(val) = vec.pop() {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));
                        node.borrow_mut().right = Some(right.clone());
                        new_nodes.push(right);
                    }
                }
                nodes = new_nodes;
            }

            return root;
        } else {
            return Rc::new(RefCell::new(TreeNode::new(0)));
        }
    }
}

pub fn to_vec(root: Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut nodes = vec![root];
    let mut result = vec![];
    'outer: loop {
        for node in nodes.iter() {
            result.push(node.borrow_mut().val);
        }

        let mut new_nodes = vec![];
        for node in nodes.iter() {
            if let Some(left) = &node.borrow_mut().left {
                new_nodes.push(left.clone());
            } else {
                break 'outer;
            }
            if let Some(right) = &node.borrow_mut().right {
                new_nodes.push(right.clone());
            }
        }
        nodes = new_nodes;
    }
    result
}

struct Solution {}

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut is_odd = false;
        if let Some(node) = &root {
            let mut nodes = vec![node.clone()];
            'outer: loop {
                is_odd = !is_odd;
                let mut new_nodes = vec![];
                for node in nodes.iter() {
                    if let Some(left) = &node.borrow_mut().left {
                        new_nodes.push(left.clone());
                    } else {
                        break 'outer;
                    }
                    if let Some(right) = &node.borrow_mut().right {
                        new_nodes.push(right.clone());
                    }
                }
                nodes = new_nodes;
                if is_odd {
                    let nums = nodes
                        .iter()
                        .map(|node| node.borrow_mut().val)
                        .collect::<Vec<_>>();

                    for (i, node) in nodes.iter().rev().enumerate() {
                        node.borrow_mut().val = nums[i];
                    }
                }
            }
        }
        root
    }
}

#[test]
fn test_1() {
    let root = TreeNode::from_vec(vec![2, 3, 5, 8, 13, 21, 34]);
    let as_vec = to_vec(root.clone());

    Solution::reverse_odd_levels(Some(TreeNode::from_vec(vec![2, 3, 5, 8, 13, 21, 34])));
}

// struct MyTree {
//     val: i32,
//     children: Option<Box<(MyTree, MyTree)>>,
// }

// #[test]
// fn use_my_tree() {
//     let tree = MyTree {
//         val: 123,
//         children: Some(Box::new((
//             MyTree {
//                 val: 234,
//                 children: None,
//             },
//             MyTree {
//                 val: 789,
//                 children: None,
//             },
//         ))),
//     };

//     if let Some(children) = tree.children {
//         let (left, right) = *children;
//         println!("Left: {}, Right: {}", left.val, right.val);
//     }
// }

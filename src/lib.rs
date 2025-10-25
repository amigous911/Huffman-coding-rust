#[derive(Debug)]
struct TreeNode {
    value: Option<u8>,
    freq: usize,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(
        value: Option<u8>,
        freq: usize,
        left: Option<TreeNode>,
        right: Option<TreeNode>,
    ) -> TreeNode {
        TreeNode {
            value: value,
            freq: freq,
            left: match left {
                Some(value) => Some(Box::new(value)),
                None => None,
            },
            right: match right {
                Some(value) => Some(Box::new(value)),
                None => None,
            },
        }
    }
}

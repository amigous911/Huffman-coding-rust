#[derive(Debug)]
pub struct TreeNode {
    pub value: Option<u8>,
    freq: usize,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

pub fn generate_tree(mut data: Vec<u8>) -> Result<TreeNode, &'static str> {
    // sort data for faster search later
    data.sort();

    // generating Huffman-tree nodes in a vector
    let mut nodes_vec: Vec<TreeNode> = Vec::new();
    let mut start_pos: usize = 0;
    while start_pos < data.len() {
        let value = data[start_pos];
        let mut end_pos: usize = start_pos + 1;

        while end_pos < data.len() && data[end_pos] == value {
            end_pos += 1
        }

        nodes_vec.push(TreeNode::new(Some(value), end_pos - start_pos, None, None));
        start_pos = end_pos;
    }

    // merging nodes (create tree)
    nodes_vec.sort_by_key(|k| k.freq);
    while nodes_vec.len() >= 2 {
        let node1 = nodes_vec.remove(0);
        let node2 = nodes_vec.remove(0);
        let new_node = TreeNode {
            value: None,
            freq: node1.freq + node2.freq,
            left: Some(Box::new(node1)),
            right: Some(Box::new(node2)),
        };
        // insert new node (node1+node2) to vector while keeping vector sorted
        match nodes_vec.binary_search_by_key(&new_node.freq, |n| n.freq) {
            Ok(pos) | Err(pos) => {
                nodes_vec.insert(pos, new_node);
            }
        }
    }

    // returning tree root
    if nodes_vec.len() != 1 {
        return Err("error generating Huffman-tree");
    }
    Ok(nodes_vec.remove(0))
}

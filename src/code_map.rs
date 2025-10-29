use crate::bit_writer::BitWriter;
use crate::huffman_tree::TreeNode;
use std::collections::HashMap;

pub struct Code {
    pub bits: u32,
    pub len: u8,
}

pub fn create_map_table(tree: &TreeNode) -> HashMap<u8, Code> {
    let mut result: HashMap<u8, Code> = HashMap::new();
    let mut writer = BitWriter::new();

    traverse(tree, &mut result, &mut writer);

    result
}

fn traverse(root: &TreeNode, map: &mut HashMap<u8, Code>, bit_writer: &mut BitWriter) {
    if root.left.is_none() && root.right.is_none() {
        map.insert(
            root.value.unwrap(),
            Code {
                bits: bit_writer.get_bits(),
                len: bit_writer.get_len(),
            },
        );

        bit_writer.delete_last_bit();
        return;
    }

    if let Some(left) = root.left.as_ref() {
        bit_writer.write_bit(0);
        traverse(&left, map, bit_writer);
    }
    if let Some(right) = root.right.as_ref() {
        bit_writer.write_bit(1);
        traverse(&right, map, bit_writer);
    }
    bit_writer.delete_last_bit();
}

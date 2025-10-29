/*
*   compressed file struct:
*   4 bytes -> size of serialized tree (in bits)
*   x bytes -> serialized tree
*   1 byte  -> number of valid bits in the last byte
*   x bytes -> actual data
*/

mod bit_writer;
mod code_map;
mod huffman_tree;

pub fn encode(data: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut encoded_data: Vec<u8> = Vec::new();
    let mut bit_writer = bit_writer::BitWriter::new();

    // generate tree
    let tree: huffman_tree::TreeNode;
    match huffman_tree::generate_tree(data.clone()) {
        Ok(value) => tree = value,
        Err(_) => return Err("error generating huffman tree"),
    }

    // serialize and store tree
    serialize_tree(&tree, &mut bit_writer);
    encoded_data.extend(bit_writer.get_len().to_le_bytes());
    encoded_data.extend(bit_writer.get_bits_buffer());

    // map values to codes
    let map = code_map::create_map_table(&tree);

    // encoding
    for byte in &data {
        let code = map.get(byte).unwrap();
        bit_writer.write_bits(code.bits, code.len);
    }

    // write last byte length and encoding result
    encoded_data.push(bit_writer.get_len() as u8);
    encoded_data.extend(bit_writer.get_bits_buffer());

    Ok(encoded_data)
}

fn serialize_tree(root: &huffman_tree::TreeNode, bit_writer: &mut bit_writer::BitWriter) {
    match root.value {
        Some(value) => {
            bit_writer.write_bit(1);
            bit_writer.write_bits(value as u32, 8);
        }
        None => {
            bit_writer.write_bit(0);
            if let Some(left) = root.left.as_ref() {
                serialize_tree(&left, bit_writer);
            }
            if let Some(right) = root.right.as_ref() {
                serialize_tree(&right, bit_writer);
            }
        }
    }
}

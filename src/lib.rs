/*
*   compressed file struct:
*   4 bytes -> size of serialized tree (in bits)
*   x bytes -> serialized tree
*   1 byte  -> number of valid bits in the last byte
*   x bytes -> actual data
*/

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

mod bit_reader;
mod bit_writer;
mod code_map;
mod huffman_tree;

pub fn encode(data: Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let mut encoded_data: Vec<u8> = Vec::new();
    let mut bit_writer = bit_writer::BitWriter::new();

    // generate tree
    let tree = huffman_tree::generate_tree(data.clone())?;

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

pub fn decode(mut data: BufReader<File>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decoded_data: Vec<u8> = Vec::new();

    let mut tree_bytes: Vec<u8>;
    let mut encoded_data: Vec<u8> = Vec::new();
    let last_encoded_byte: Option<u8>;
    let mut last_byte_len = [0u8; 1];

    // read data
    {
        // tree
        let mut tree_bit_len = [0u8; 4];
        data.read_exact(&mut tree_bit_len)?;
        let tree_bit_len = u32::from_le_bytes(tree_bit_len);

        if tree_bit_len == 0 {
            return Err(Box::from("tree length can't be zero"));
        }

        let tree_byte_len = ((tree_bit_len + 7) / 8) as usize;
        tree_bytes = vec![0u8; tree_byte_len];
        data.read_exact(&mut tree_bytes)?;

        // last byte length
        data.read_exact(&mut last_byte_len)?;

        // encoded data
        data.read_to_end(&mut encoded_data)?;
        if last_byte_len[0] == 0 {
            last_encoded_byte = None
        } else {
            last_encoded_byte = encoded_data.pop();
        }
    }

    // deserialzie tree
    let mut bit_reader = bit_reader::BitReader::new(tree_bytes)?;
    let tree = deserialize_tree(&mut bit_reader);

    // decode
    let mut bit_reader = bit_reader::BitReader::new(encoded_data)?;
    let mut current_node = &tree;

    loop {
        if let Some(node_value) = current_node.value {
            decoded_data.push(node_value);
            current_node = &tree;
        }

        match bit_reader.get_bit() {
            None => break,
            Some(value) => {
                if value {
                    if let Some(right) = current_node.right.as_ref() {
                        current_node = &right;
                    }
                } else {
                    if let Some(left) = current_node.left.as_ref() {
                        current_node = &left;
                    }
                }
            }
        }
    }

    if let Some(mut last_byte) = last_encoded_byte {
        for _ in 0..last_byte_len[0] {
            if let Some(node_value) = current_node.value {
                decoded_data.push(node_value);
                current_node = &tree;
            }

            if last_byte & 0b10000000 == 0 {
                if let Some(left) = current_node.left.as_ref() {
                    current_node = &left;
                }
            } else {
                if let Some(right) = current_node.right.as_ref() {
                    current_node = &right;
                }
            }
            last_byte <<= 1;
        }
        decoded_data.push(current_node.value.unwrap());
    }

    Ok(decoded_data)
}

fn deserialize_tree(bit_reader: &mut bit_reader::BitReader) -> huffman_tree::DeserializedTreeNode {
    if bit_reader.get_bit().unwrap() {
        huffman_tree::DeserializedTreeNode::new(bit_reader.get_byte(), None, None)
    } else {
        let left = deserialize_tree(bit_reader);
        let right = deserialize_tree(bit_reader);
        huffman_tree::DeserializedTreeNode::new(None, Some(left), Some(right))
    }
}

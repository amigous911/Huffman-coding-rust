pub struct BitWriter {
    buffer: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

impl BitWriter {
    pub fn new() -> BitWriter {
        BitWriter {
            buffer: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn write_bits(&mut self, bits: u32, len: u8) {
        for i in (0..len).rev() {
            let bit = (bits >> i) & 1;
            self.write_bit(bit as u8);
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        self.current_byte <<= 1;
        self.current_byte |= bit;
        self.bit_pos += 1;

        // push to buffer if byte is full
        if self.bit_pos == 8 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }

    pub fn delete_last_bit(&mut self) {
        // do nothing if there are no bits
        if self.bit_pos == 0 && self.buffer.len() == 0 {
            return;
        }

        // pull last byte from buffer if current byte is empty
        if self.bit_pos == 0 {
            self.current_byte = self.buffer.pop().unwrap();
            self.bit_pos = 7;
        }

        // deleting bit
        self.current_byte >>= 1;
        self.bit_pos -= 1;
    }

    fn get_bits(&self) -> u32 {
        let mut result: u32 = 0;

        if self.buffer.len() == 0 {
            return self.current_byte as u32;
        }

        for (index, byte) in self.buffer.iter().enumerate() {
            result |= *byte as u32;
            if index != self.buffer.len() - 1 {
                result <<= 8;
            }
        }

        result
    }

    fn get_len(&self) -> u8 {
        (self.buffer.len() * 8) as u8 + self.bit_pos
    }
    
    /// returns a copy of the buffer so the original one retain its values
    pub fn clone_bits_buffer(&mut self) -> Vec<u8> {
        let mut result = self.buffer.clone();
        if self.bit_pos > 0 {
            self.current_byte <<= 8 - self.bit_pos;
            result.push(self.current_byte);
            self.current_byte >>= 8 - self.bit_pos;
        }
        result
    }

    /// returns the buffer and replaces it with an empty one
    pub fn get_bits_buffer(&mut self) -> Vec<u8> {
        if self.bit_pos > 0 {
            self.current_byte <<= 8 - self.bit_pos;
            self.buffer.push(self.current_byte);
        }
        let result = std::mem::take(&mut self.buffer);
        self.buffer = Vec::new();
        self.current_byte = 0;
        self.bit_pos = 0;
        result
    }
}

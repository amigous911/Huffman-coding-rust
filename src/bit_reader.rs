struct BitReader {
    buffer: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

impl BitReader {
    fn new(buffer: Vec<u8>) -> Result<BitReader, &'static str> {
        if buffer.len() == 0 {
            return Err("buffer length can't be zero");
        }

        Ok(BitReader {
            buffer: buffer,
            current_byte: 0,
            bit_pos: 0,
        })
    }

    fn is_empyt(&self) -> bool {
        self.buffer.len() == 0 && self.bit_pos == 0
    }

    fn get_byte(&mut self) -> Option<u8> {
        if self.buffer.len() == 0 {
            return None;
        }

        let mut result: u8 = 0;
        for _ in 0..8 {
            result <<= 1;
            if self.get_bit().unwrap() {
                result |= 1;
            }
        }
        Some(result)
    }

    fn get_bit(&mut self) -> Option<bool> {
        if self.is_empyt() {
            return None;
        }

        if self.bit_pos == 0 {
            self.current_byte = self.buffer.remove(0);
            self.bit_pos = 8;
        }

        self.bit_pos -= 1;
        let result: u8 = self.current_byte & 0x80;
        self.current_byte <<= 1;
        Some(if result == 0 { false } else { true })
    }
}

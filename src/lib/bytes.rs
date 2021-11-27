use std::ops::{Index, Range, RangeFrom};

#[derive(Debug, PartialEq)]
pub struct Bytes<'a> {
    bytes: &'a mut [u8],
}

impl<'a> Bytes<'a> {
    pub fn from(bytes: &'a mut [u8]) -> Bytes<'a> {
        Bytes { bytes }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Reads unsigned 32bit little endian integer and iterates offset by 4.
    pub fn read_u32(&self, offset: &mut usize) -> u32 {
        let array = [
            self.bytes[*offset],
            self.bytes[*offset+1],
            self.bytes[*offset+2],
            self.bytes[*offset+3]
        ];
        *offset = *offset + 4;
        u32::from_le_bytes(array)
    }

    /// Reads unsigned 16bit little endian integer and iterates offset by 2.
    pub fn read_u16(&self, offset: &mut usize) -> u16 {
        let array: [u8; 2] = [self.bytes[*offset], self.bytes[*offset + 1]];
        *offset = *offset + 2;
        u16::from_le_bytes(array)
    }

    pub fn as_text(&self) -> Option<&str> {
        core::str::from_utf8(&self.bytes).ok()
    }


    pub fn decrypt(&mut self, encryption_key: &[u8]) {

        /*
        1. Subtract the character value from the current position in the encryption key
        2. Advance the position in the encryption key by one
        3. If the end of the encryption key has been reached, go back to the first character
        4. Subtract the value of the previous byte read
        5. Logical AND with 0xFF to limit the result to 0-255
        6. This byte is now decoded, move on to the next
        */
        
        // The position in the encryption key does not start at 0. Instead, it starts at 25 modulo <length of key>.
        let mut key_index: usize = 25 % encryption_key.len();

        // The very first "previous byte read" value is the actual key character at the initial position.
        let mut previous_byte_read = encryption_key[key_index];

        for ix in 0..self.bytes.len() {

            let mut decrypted_byte = self.bytes[ix] as i32;
            decrypted_byte = decrypted_byte - (encryption_key[key_index] as i32);
            decrypted_byte = decrypted_byte - previous_byte_read as i32;
            decrypted_byte = decrypted_byte & 0xFF;
            
            previous_byte_read = self.bytes[ix];

            self.bytes[ix] = decrypted_byte as u8;

            key_index = key_index + 1;
            if key_index == encryption_key.len() {
                key_index = 0;
            }
        }
    }
}

impl<'a> Index<usize> for Bytes<'a> {
    type Output = u8;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.bytes[ix]
    }
}


impl<'a> Index<Range<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, ix: Range<usize>) -> &Self::Output {
        &self.bytes[ix]
    }
}


impl<'a> Index<RangeFrom<usize>> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, ix: RangeFrom<usize>) -> &Self::Output {
        &self.bytes[ix]
    }
}

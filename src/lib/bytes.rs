use std::ops::{Index, RangeFrom};

use super::utils::as_u32_le;

#[derive(Debug, PartialEq, Clone)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn from(bytes: Vec<u8>) -> Bytes {
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
}

impl Index<usize> for Bytes {
    type Output = u8;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.bytes[ix]
    }
}

impl Index<RangeFrom<usize>> for Bytes {
    type Output = [u8];

    fn index(&self, ix: RangeFrom<usize>) -> &Self::Output {
        &self.bytes[ix]
    }
}

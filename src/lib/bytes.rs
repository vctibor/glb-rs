use std::ops::{Index, RangeFrom};

use super::utils::as_u32_le;

#[derive(Debug, PartialEq, Clone)]
pub struct Bytes {
    bytes: Vec<u8>
}

impl Bytes {

    pub fn from(bytes: Vec<u8>) -> Bytes { Bytes { bytes } }

    pub fn len(&self) -> usize { self.bytes.len() }

    /// Reads unsigned 32bit little endian integer and iterates offset by 4.
    pub fn read_u32(&self, offset: &mut usize) -> u32 {
        let number_bytes = &self.bytes[*offset..*offset+4];
        *offset = *offset + 4;
        as_u32_le(number_bytes)
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
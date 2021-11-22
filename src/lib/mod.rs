mod model;
mod utils;
#[cfg(test)] mod test;

use model::*;
use utils::*;

const ENCRYPTION_KEY: &[u8; 8] = b"32768GLB";

// The game only decrypts one FAT entry at a time, so after 28 bytes the key and "previous byte read" must be set back to the initial state.
const CHUNK_SIZE: usize = 28;

fn decrypt(encrypted_bytes: &[u8], encryption_key: &[u8]) -> [u8; CHUNK_SIZE] {

    if encrypted_bytes.len() != CHUNK_SIZE {
        panic!("You should decrypt by chunks of 28 bytes.");
    }

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
    
    let mut decrypted_bytes: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    for ix in 0..CHUNK_SIZE {

        let mut decrypted_byte = encrypted_bytes[ix] as i32;
        decrypted_byte = decrypted_byte - (encryption_key[key_index] as i32);
        decrypted_byte = decrypted_byte - previous_byte_read as i32;
        decrypted_byte = decrypted_byte & 0xFF;
        
        previous_byte_read = encrypted_bytes[ix];
        decrypted_bytes[ix] = decrypted_byte as u8;
        key_index = key_index + 1;
        if key_index == encryption_key.len() {
            key_index = 0;
        }
    }

    decrypted_bytes
}

// Parses single 28 byte File Allocation Table entry.
fn parse_fat_entry(encrypted_bytes: &[u8]) -> FatEntry {

    let decrypted_bytes = decrypt(encrypted_bytes, ENCRYPTION_KEY);

    let flag = match as_u32_le(&decrypted_bytes[0..4]) {
        0 => Flag::Normal,
        1 => Flag::Encrypted,
        other => panic!("Unknown flag {}", other)
    };

    let offset = as_u32_le(&decrypted_bytes[4..8]);

    let length = as_u32_le(&decrypted_bytes[8..12]);

    let filename = core::str::from_utf8(&decrypted_bytes[12..28]).unwrap().to_string();

    FatEntry { flag, offset, length, filename }
}

// If given first 28 bytes of file it parses them as header,
// that means that offset field is interpreted as number of files,
// which is value returned, and other fields are ignored.
fn parse_header(encrypted_bytes: &[u8]) -> usize {
    let entry = parse_fat_entry(encrypted_bytes);
    entry.offset as usize
}

pub fn parse_fat(encrypted_bytes: &[u8]) -> FileAllocationTable {

    let mut offset: usize = 0;

    let fat_entries_count = parse_header(&encrypted_bytes[offset..offset+CHUNK_SIZE]);

    let mut entries = Vec::with_capacity(fat_entries_count);

    for _ in 0..fat_entries_count {
        offset = offset + CHUNK_SIZE;
        let entry = parse_fat_entry(&encrypted_bytes[offset..offset+CHUNK_SIZE]);
        entries.push(entry);
    }

    FileAllocationTable { entries }
}


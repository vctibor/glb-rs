/// Interprets array of four bytes as little endian unsigned 32bit integer.
pub fn as_u32_le(array: &[u8]) -> u32 {
    // https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

pub fn decrypt(encrypted_bytes: &[u8], encryption_key: &[u8]) -> Vec<u8> {

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
    
    let mut decrypted_bytes: Vec<u8> = Vec::with_capacity(encrypted_bytes.len());

    for ix in 0..encrypted_bytes.len() {

        let mut decrypted_byte = encrypted_bytes[ix] as i32;
        decrypted_byte = decrypted_byte - (encryption_key[key_index] as i32);
        decrypted_byte = decrypted_byte - previous_byte_read as i32;
        decrypted_byte = decrypted_byte & 0xFF;
        
        previous_byte_read = encrypted_bytes[ix];
        
        decrypted_bytes.push(decrypted_byte as u8);

        key_index = key_index + 1;
        if key_index == encryption_key.len() {
            key_index = 0;
        }
    }

    decrypted_bytes
}
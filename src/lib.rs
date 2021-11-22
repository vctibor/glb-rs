pub const ENCRYPTION_KEY: &[u8; 8] = b"32768GLB";

// The game only decrypts one FAT entry at a time, so after 28 bytes the key and "previous byte read" must be set back to the initial state.
const DECRYPTION_CHUNK_SIZE: usize = 28;

pub fn decrypt(encrypted_bytes: &[u8], encryption_key: &[u8]) -> [u8; DECRYPTION_CHUNK_SIZE] {

    //if encrypted_bytes.len() != DECRYPTION_CHUNK_SIZE {
    //    panic!("You should decrypt by chunks of 28 bytes.");
    //}

    /*
    1. Subtract the character value from the current position in the encryption key
    2. Advance the position in the encryption key by one
    3. If the end of the encryption key has been reached, go back to the first character
    4. Subtract the value of the previous byte read
    5. Logical AND with 0xFF to limit the result to 0-255
    6. This byte is now decoded, move on to the next
    */
    
    // The position in the encryption key does not start at 0. Instead, it starts at 25 modulo <length of key>.
    //let mut key_index: usize = ((DECRYPTION_CHUNK_SIZE as i32) % encryption_key.len() as i32) as usize;

    //let mut key_index: usize = DECRYPTION_CHUNK_SIZE.rem_euclid(encryption_key.len());

    let mut key_index: usize = 1;
    
    // The very first "previous byte read" value is the actual key character at the initial position.
    let mut previous_byte_read = encryption_key[key_index];
    
    let mut decrypted_bytes: [u8; DECRYPTION_CHUNK_SIZE] = [0; DECRYPTION_CHUNK_SIZE];


    //println!("encryption key {:?}", ENCRYPTION_KEY);

    for ix in 0..DECRYPTION_CHUNK_SIZE {
        //let decrypted_byte = ((encryption_key[key_index] - encrypted_bytes[ix]) - previous_byte_read) & 0xFF;
        
        
        //let decrypted_byte = ((encrypted_bytes[ix] - encryption_key[key_index]) - previous_byte_read) & 0xFF;

        //println!("{}", encrypted_bytes[ix]);

        //let mut decrypted_byte = encryption_key[key_index] as i32;
        let mut decrypted_byte = encrypted_bytes[ix] as i32;
        //println!("{}", decrypted_byte);



        //println!("key index: {}", key_index);
        //println!("encryption key byte: {}", encryption_key[key_index]);


        //decrypted_byte = decrypted_byte - encrypted_bytes[ix] as i32;
        decrypted_byte = decrypted_byte - (encryption_key[key_index] as i32);
        //println!("{}", decrypted_byte);

        decrypted_byte = decrypted_byte - previous_byte_read as i32;
        //println!("{}", decrypted_byte);

        decrypted_byte = decrypted_byte & 0xFF;
        //println!("{}", decrypted_byte);

        //println!("");

        
        previous_byte_read = encrypted_bytes[ix];
        decrypted_bytes[ix] = decrypted_byte as u8;
        key_index = key_index + 1;
        if key_index == encryption_key.len() {
            key_index = 0;
        }
    }

    decrypted_bytes
}

// https://stackoverflow.com/questions/36669427/does-rust-have-a-way-to-convert-several-bytes-to-a-number
pub fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

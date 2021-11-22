use glb_rs::*;

pub fn main() {
    
    //let test = [0x64, 0x9B, 0xD1, 0x09];
    //glb_rs::decrypt(&test, glb_rs::ENCRYPTION_KEY);
    
    
    
    
    let bytes = std::fs::read("test_files/FILE0000.GLB").unwrap();

    let first_chunk: &[u8] = &bytes[0..28];


    let decrypted = glb_rs::decrypt(first_chunk, glb_rs::ENCRYPTION_KEY);

    println!("{:?}", decrypted);
}
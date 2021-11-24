use super::*;
use utils::*;

#[test]
fn decrypt_test_1() {
    let mut encrypted: [u8; 28] = [0; 28];
    encrypted[0] = 0x64;
    encrypted[1] = 0x9B;
    encrypted[2] = 0xD1;
    encrypted[3] = 0x09;
    let decrypted = decrypt(&encrypted, glb_archive::ENCRYPTION_KEY);
    assert_eq!(decrypted[0], 0);
    assert_eq!(decrypted[1], 0);
    assert_eq!(decrypted[2], 0);
    assert_eq!(decrypted[3], 0);
}
pub fn main() {
    let bytes = std::fs::read("test_files/FILE0000.GLB").unwrap();
    let fat_entry = glb_rs::parse_fat(&bytes);
    println!("{:#?}", fat_entry);
}
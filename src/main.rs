use glb_rs::GlbArchive;

pub fn main() {
    let bytes = std::fs::read("test_files/FILE0001.GLB").unwrap();
    //let fat = glb_rs::parse_fat(&bytes);

    let archive = GlbArchive::new(&bytes);

    let fat = archive.parse_fat();

    archive.extract_files(&fat);

    /*
    for ix in 0..fat.entries.len() {
        println!("{} - {} - {}", ix, fat.entries[ix].filename, fat.entries[ix].length);
    }

    let palette = glb_rs::get_DAT(glb_rs::read_file(&bytes, &fat.entries[0]));


    let bytes = std::fs::read("test_files/FILE0002.GLB").unwrap();
    let fat = glb_rs::parse_fat(&bytes);

    for ix in 0..fat.entries.len() {
        println!("{} - {} - {}", ix, fat.entries[ix].filename, fat.entries[ix].length);
    }

    let pic = glb_rs::get_PIC(glb_rs::read_file(&bytes, &fat.entries[241]), &palette);
    */
}
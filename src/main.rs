use glb_rs::{ GlbArchive, File };

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

pub fn main() {
    let bytes = std::fs::read("test_files/FILE0001.GLB").unwrap();
    //let fat = glb_rs::parse_fat(&bytes);

    let archive = GlbArchive::new(&bytes);

    let fat = archive.parse_fat();

    let files = archive.extract_files(&fat);

    //println!("{:?}", files.keys());

    //let pic = files.get("HANGER_PIC").unwrap().clone();

    for file in files.values() {

        match file {
            File::Pic(p) => {
                let width = p.width as u32;
                let height = p.height as u32;
                
                // println!("{} {}", p.pixels.len(), p.width*p.height);
                
                let mut pixels = p.pixels.clone().into_iter();

                let mut img: RgbaImage = ImageBuffer::new(width, height);

                for y in 0..height {
                    for x in 0..width {
                        if let Some(pixel) = pixels.next() {
                            let r = pixel.red;
                            let g = pixel.green;
                            let b = pixel.blue;
                            let a = pixel.alpha;
                            img.put_pixel(x, y, Rgba([r, g, b, a]));
                        }
                    }
                }

                let export_path = format!("export/{}.png", p.filename);

                let _ = img.save(export_path);
            }
            _ => {}
        }
    }


    

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
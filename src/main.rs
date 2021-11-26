use std::io::Write;

use glb_rs::*;
use image::{ImageBuffer, Rgba, RgbaImage, imageops::{self}};
use image::imageops::FilterType;
use image::imageops::resize;

const EXPORT_FOLDER: &'static str = "./export";

pub fn main() {

    let _ = std::fs::remove_dir_all(EXPORT_FOLDER);
    let _ = std::fs::create_dir_all(EXPORT_FOLDER);
    
    let palette = {
            
        let bytes = std::fs::read("test_files/FILE0001.GLB").unwrap();
        let archive = GlbArchive::new(&bytes);
        let fat = archive.parse_fat();
        let files = archive.extract_files(&fat);

        let palette = files.get("PALETTE_DAT").unwrap().clone();
        
        let palette = match palette {
            File::Palette(p) => p,
            _ => panic!("PALETTE_DAT has to be palette!")
        };

        palette
    };

    let bytes = std::fs::read("test_files/FILE0003.GLB").unwrap();
    let archive = GlbArchive::new(&bytes);
    let fat = archive.parse_fat();
    let files = archive.extract_files(&fat);

    for file in files.values() {
        match file {
            File::Map(m) => {
                //println!("{} {}", m.filename, m.actor_count);
            }
            
            File::Text(t) => {
                let export_path = format!("{}/{}.txt", EXPORT_FOLDER, t.filename);
                save_text(t, &export_path);
            }
            File::Pic(p) => {
                let export_path = format!("{}/{}.png", EXPORT_FOLDER, p.filename);
                save_pic(p, &palette, &export_path);
            }
            File::Tiles(t) => {
                save_tiles(t);
            }
            
            _ => {}
        }
    }
}

fn save_tiles(t: &Tiles) {
    let mut ix = 0;
    for tile in &t.tiles {
        let path = format!("{}/_tile{}.png", EXPORT_FOLDER, ix);
        ix = ix + 1;
        //save_pic(tile, &path);
    }
}

fn save_pic(p: &Pic, palette: &Palette, export_path: &str) {
    let width = p.width as u32;
    let height = p.height as u32;

    let mut pixels = p.get_argb(palette).into_iter();

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
    
    let img = resize(&img, width*4, height*4, FilterType::CatmullRom);

    let _ = img.save(export_path);
}

fn save_text(t: &Text, export_path: &str) {
    let mut f = std::fs::File::create(export_path).unwrap();
    f.write_all(t.text.as_bytes()).unwrap();
}

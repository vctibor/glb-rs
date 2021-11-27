use std::io::Write;
use std::time::Instant;

use glb_rs::*;
use image::imageops::overlay;
use image::{ImageBuffer, RgbaImage};

const EXPORT_FOLDER: &'static str = "./export";

pub fn main() {


    //export();
    measure();
}

fn measure() {

    for _ in 0..10 {
        let now = Instant::now();

        let mut archive = GlbArchive::from_file("test_files/FILE0001.GLB").unwrap();
        let fat = archive.parse_fat();
        let files = archive.extract_files(&fat);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
}

fn export() {
    let now = Instant::now();

    let _ = std::fs::remove_dir_all(EXPORT_FOLDER);
    let _ = std::fs::create_dir_all(EXPORT_FOLDER);

    let palette = {
        let mut archive =  GlbArchive::from_file("test_files/FILE0001.GLB").unwrap();
        let fat = archive.parse_fat();
        let files = archive.extract_files(&fat);

        let palette = files.named_files.get("PALETTE_DAT").unwrap().clone();

        match palette {
            File::Palette(p) => p,
            _ => panic!("PALETTE_DAT has to be palette!"),
        }
    };

    let mut archive = GlbArchive::from_file("test_files/FILE0001.GLB").unwrap();
    let fat = archive.parse_fat();
    let extracted = archive.extract_files(&fat);

    let tiles = extracted.tiles;

    for file in extracted.named_files.values() {
        match file {
            File::Map(m) => {
                save_map(&m, &tiles, &palette);
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
                save_tiles(t, &palette);
            }
            _ => {}
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn save_map(m: &Map, tiles: &Tiles, palette: &Palette) {
    let tile_size = 32;

    let image_width = m.width as u32 * tile_size;
    let image_height = m.height as u32 * tile_size;

    let mut img: RgbaImage = ImageBuffer::new(image_width, image_height);

    for y in 0..m.height {
        for x in 0..m.width {

            let tile_ix = m.tiles[y][x] as usize;

            if tile_ix >= tiles.tiles.len() {
                continue;
            }

            let tile = tiles.tiles[tile_ix].clone();

            let on_top = tile.to_imagebuffer(palette);


            
                
            let image_x = x as u32 * tile_size;
            let image_y = y as u32 * tile_size;

            /*
            let mut image_x = (x as u32 + 1) * tile_size;
            let mut image_y = y as u32 * tile_size;

            if x == m.width - 1 {
                image_x = 0;

                image_y = image_y + tile_size;
            }
            */
            
            overlay(&mut img, &on_top, image_x, image_y);
        }
    }

    let path = format!("{}/{}.png", EXPORT_FOLDER, m.filename);
    let _ = img.save(path);
}

fn save_tiles(t: &Tiles, palette: &Palette) {
    let mut ix = 0;
    for tile in &t.tiles {
        let path = format!("{}/_tile{}.png", EXPORT_FOLDER, ix);
        ix = ix + 1;
        save_pic(tile, palette, &path);
    }
}

fn save_pic(p: &Pic, palette: &Palette, export_path: &str) {
    let img = p.to_imagebuffer(palette);
    //let img = resize(&img, width*4, height*4, FilterType::Nearest);
    let _ = img.save(export_path);
}

fn save_text(t: &Text, export_path: &str) {
    let mut f = std::fs::File::create(export_path).unwrap();
    f.write_all(t.text.as_bytes()).unwrap();
}

use image::{ImageBuffer, RgbaImage, Rgba};

use super::utils::*;
use super::glb_archive::*;
use super::bytes::Bytes;

const MAP_WIDTH: usize = 9;
const MAP_HEIGHT: usize = 150;

#[derive(Debug, PartialEq, Clone)]
pub struct Text {
    pub filename: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ArgbPixel {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[derive(Debug, PartialEq, Clone)]
pub struct Palette {
    pub filename: String,
    pub palette:  Vec<ArgbPixel>
}

/// https://moddingwiki.shikadi.net/wiki/Raptor_PIC_Format
#[derive(Debug, PartialEq, Clone)]
pub struct Pic {
    pub filename: String,
    pub width:  usize,
    pub height: usize,

    /// Indexes into palette. Palette has to be provided to get RGB values.
    /// Option::None indicates transparent pixel.
    pub pixels: Vec<Option<u8>>,
}

impl Pic {
    pub fn get_argb(&self, palette: &Palette) -> Vec<ArgbPixel> {
        let mut argb_pixels = Vec::with_capacity(self.pixels.len());
        for pix in &self.pixels {
            match pix {
                None => argb_pixels.push(ArgbPixel { alpha: 0, red: 0, green: 0, blue: 0 }),
                Some(palette_ix) => {
                    let palette_ix = *palette_ix as usize;
                    argb_pixels.push(palette.palette[palette_ix]);
                }
            }
        }
        argb_pixels
    }

    pub fn to_imagebuffer(&self, palette: &Palette) -> RgbaImage {
        let width = self.width as u32;
        let height = self.height as u32;

        let mut pixels = self.get_argb(palette).into_iter();

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

        img
    }
}

/// https://moddingwiki.shikadi.net/wiki/Raptor_Level_Format
#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub filename: String,

    pub width: usize,
    pub height: usize,

    pub actor_count: u32,

    /// Index into tileset
    pub tiles: [[u16; MAP_WIDTH]; MAP_HEIGHT],
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tiles {
    pub tiles: Vec<Pic>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum File {
    Text(Text),
    Palette(Palette),
    Pic(Pic),
    Map(Map),
    Tiles(Tiles),
}


/// Represents file that was extracted from GLB and decrypted if necessary.
#[derive(Debug, PartialEq, Clone)]
pub struct UntypedFile {
    bytes: Bytes,     // TODO: should use reference
    pub filename: String,
}

impl UntypedFile {

    fn new(bytes: Vec<u8>, filename: String) -> UntypedFile {
        UntypedFile { bytes: Bytes::from(bytes), filename }
    }


    /// Copy bytes representing single file from slice into new vector.
    /// If the file is encrypted, this function performs decryption.
    pub fn read_file(archive: &GlbArchive, entry: &FatEntry) -> UntypedFile {
        let length = entry.length as usize;
        let offset = entry.offset as usize;
        let mut file: Vec<u8> = Vec::with_capacity(length);
        for ix in offset..offset+length {
            file.push(archive.bytes[ix]);
        }
        if entry.flag == Flag::Encrypted {
            file = decrypt(&file, ENCRYPTION_KEY);
        }
        UntypedFile::new(file, entry.filename.clone())
    }

    pub fn get_txt(&self) -> Option<Text> {
        self.bytes.as_text()
            .map(|s| Text { filename: self.filename.clone(), text: s.to_owned() })
    }

    /// Parses VGA pallete.
    /// https://moddingwiki.shikadi.net/wiki/VGA_Palette
    pub fn get_dat(&self) -> Option<Palette> {

        let mut palette: Vec<ArgbPixel> = Vec::with_capacity(self.bytes.len() / 2);

        for ix in (0..self.bytes.len()).step_by(3) {
            let red = ((self.bytes[ix] as u32 * 255) / 63) as u8;
            let green = ((self.bytes[ix+1] as u32 * 255) / 63) as u8;
            let blue = ((self.bytes[ix+2] as u32 * 255) / 63) as u8;
            palette.push(ArgbPixel { alpha: 255, red, green, blue });
        }

        Some(Palette { filename: self.filename.clone(), palette })
    }

    /// Parses Raptor PIC format.
    /// https://moddingwiki.shikadi.net/wiki/Raptor_PIC_Format
    /// https://moddingwiki.shikadi.net/wiki/Raw_VGA_Image
    pub fn get_pic(&self) -> Option<Pic> {
        
        /*
        UINT32LE 	unknown1 	Always 1 when iLineCount is 0
        UINT32LE 	unknown2 	Always 1 when iLineCount is 0
        UINT32LE 	iLineCount 	Number of non-transparent image lines?
        UINT32LE 	width 	    Width of the image, in pixels
        UINT32LE 	height 	    Height of the image, in pixels
        UINT8 	    data[] 	    8bpp raw VGA data, one byte per pixel; or sprite layout blocks 
        */

        let mut offset: usize = 0;

        let _unknown_1 = self.bytes.read_u32(&mut offset);
        let _unknown_2 = self.bytes.read_u32(&mut offset);
        let i_line_count = self.bytes.read_u32(&mut offset);
        let width = self.bytes.read_u32(&mut offset) as usize;
        let height = self.bytes.read_u32(&mut offset) as usize;
        
        if i_line_count == 0 {
            let pixels: Vec<Option<u8>> = self.bytes[offset..].into_iter().map(|b| Some(*b)).collect();
            return Some(Pic { filename: self.filename.clone(), width, height, pixels });
        }

        /*
        0 | UINT32LE     | iPosX		    relative to left edge of image
        4 | UINT32LE     | iPosY		    relative to top edge of image
        8 | UINT32LE     | iLinearOffset	relative to top-left pixel
        12 | UINT32LE     | iCount		    number of pixels to write (in that row)
        16 | BYTE[iCount] | bPixels		    pixels to write
        */

        let mut pixels: Vec<Option<u8>> = vec![None; width*height];

        loop {

            let i_pos_x = self.bytes.read_u32(&mut offset) as usize;
            let i_pos_y = self.bytes.read_u32(&mut offset) as usize;
            let i_linear_offset = self.bytes.read_u32(&mut offset);
            let i_count = self.bytes.read_u32(&mut offset) as usize;

            if i_linear_offset == 0xFFFFFFFF && i_count == 0xFFFFFFFF {
                break;
            }

            let block_end = offset+i_count;
            let mut pass = 0;
            while offset < block_end {
                let palette_ix = self.bytes[offset];
                let pixels_ix = (i_pos_y*width) + i_pos_x + pass;
                pass = pass + 1;
                pixels[pixels_ix] = Some(palette_ix);
                offset = offset + 1;
            }
        }

        return Some(Pic { filename: self.filename.clone(), width, height, pixels });
    }

    
    pub fn get_map(&self) -> Option<Map> {

        /*
        0 | UINT32LE       | iFileSize    | size of the entire level file
        4 | UINT32LE       | iActorOffset | always 0x1524
        8 | UINT32LE       | iActorCount  | always (iFileSize-iActorOffset)/24
        12 | UINT32LE[1350] | iTileData    | could also be two UINT16LE per tile     
        */
        
        let filename = self.filename.clone();

        let mut offset: usize = 0;
        
        let _file_size = self.bytes.read_u32(&mut offset) as usize;
        let _actor_offset = self.bytes.read_u32(&mut offset);
        let actor_count = self.bytes.read_u32(&mut offset);
        let _tile_data = self.bytes.read_u32(&mut offset);


        let mut tiles: [[u16; MAP_WIDTH]; MAP_HEIGHT] = [[0; MAP_WIDTH]; MAP_HEIGHT];


        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {

                // let index = (y * LEVEL_HEIGHT) + x;

                let tile_number = self.bytes.read_u16(&mut offset);
                let _tileset_number = self.bytes.read_u16(&mut offset);

                //print!("[{},{}]", tile_number, tileset_number);

                tiles[y][x] = tile_number;
            }

            //println!("");
        }

        /*
        while offset < file_size {
            let tile_number = self.bytes.read_u16(&mut offset);
            let tileset_number = self.bytes.read_u16(&mut offset);

            print!("[{},{}]", tile_number, tileset_number);
        }
        */

        //println!("{:?}", tiles);

        
        let map = Map {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            filename,
            actor_count,
            tiles,
        };

        Some(map)
    }
    
    
    pub fn get_tile(&self) -> Option<Pic> {
        /*
        UINT32LE 	unknown1 	? always 1?
        UINT32LE 	unknown2 	? always 0?
        UINT32LE 	unknown3 	? always 1?
        UINT32LE 	width 	    Width of the tile, in pixels
        UINT32LE 	height 	    Height of the tile, in pixels
        UINT8 	    data[1024] 	8bpp raw VGA data, one byte per pixel; 
        */

        let filename = self.filename.clone();

        let mut offset: usize = 0;
        let _unknown_1 = self.bytes.read_u32(&mut offset);
        let _unknown_2 = self.bytes.read_u32(&mut offset);
        let _unknown_3 = self.bytes.read_u32(&mut offset);
        let width = self.bytes.read_u32(&mut offset) as usize;
        let height = self.bytes.read_u32(&mut offset) as usize;
        let data  = self.bytes[offset..].to_vec();

        let mut pixels: Vec<Option<u8>> = Vec::with_capacity(data.len() / 2);
        for palette_ix in data {
            pixels.push(Some(palette_ix))
        }

        return Some(Pic { filename, width, height, pixels });
    }
    
}
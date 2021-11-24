use super::utils::*;
use super::glb_archive::*;

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
    pub pixels: Vec<ArgbPixel>
}

#[derive(Debug, PartialEq, Clone)]
pub enum File {
    Text(Text),
    Palette(Palette),
    Pic(Pic),
}


/// Represents file that was extracted from GLB and decrypted if necessary.
#[derive(Debug, PartialEq, Clone)]
pub struct UntypedFile {
    bytes: Vec<u8>,     // TODO: should use reference
    pub filename: String,
}

impl UntypedFile {

    fn new(bytes: Vec<u8>, filename: String) -> UntypedFile {
        UntypedFile { bytes, filename }
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
        return core::str::from_utf8(&self.bytes).ok()
            .map(|s| Text { filename: self.filename.clone(), text: s.to_owned() });
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
    pub fn get_pic(&self, palette: &Palette) -> Option<Pic> {
        
        /*
        UINT32LE 	unknown1 	Always 1 when iLineCount is 0
        UINT32LE 	unknown2 	Always 1 when iLineCount is 0
        UINT32LE 	iLineCount 	Number of non-transparent image lines?
        UINT32LE 	width 	    Width of the image, in pixels
        UINT32LE 	height 	    Height of the image, in pixels
        UINT8 	    data[] 	    8bpp raw VGA data, one byte per pixel; or sprite layout blocks 
        */

        let mut offset: usize = 0; 

        let _unknown1 = as_u32_le(&self.bytes[offset..offset+4]);
        offset = offset + 4;

        let _unknown2 = as_u32_le(&self.bytes[offset..offset+4]);
        offset = offset + 4;

        let i_line_count = as_u32_le(&self.bytes[offset..offset+4]);
        //println!("i_line_count {}", i_line_count);
        offset = offset + 4;

        let width = as_u32_le(&self.bytes[offset..offset+4]) as usize;
        offset = offset + 4;

        let height = as_u32_le(&self.bytes[offset..offset+4]) as usize;
        offset = offset + 4;

        
        let data = self.bytes[23..].to_vec();
        
        
        if i_line_count == 0 {
            let mut pixels: Vec<ArgbPixel> = Vec::with_capacity(data.len() / 2);
            for palette_ix in data {
                let palette_ix = palette_ix as usize;
                pixels.push(palette.palette[palette_ix].clone())
            }
            return Some(Pic { filename: self.filename.clone(), width, height, pixels });
        }

        /*
        0 | UINT32LE     | iPosX		    relative to left edge of image
        4 | UINT32LE     | iPosY		    relative to top edge of image
        8 | UINT32LE     | iLinearOffset	relative to top-left pixel
        12 | UINT32LE     | iCount		    number of pixels to write (in that row)
        16 | BYTE[iCount] | bPixels		    pixels to write
        */

        // TODO

        return None;
    }
}
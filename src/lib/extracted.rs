use super::{File, Tiles};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Extracted {
    
    pub named_files: HashMap<String, File>,

    pub tiles: Tiles,
}

/*




#[derive(Debug, PartialEq, Clone)]
pub struct Extracted<'a> {

    /// Assigns index to file.
    /// Main storage of files.
    files: HashMap<String, File>,

    /// List of indexes of TXT files
    text_files: Vec<&'a Text>,

    /// List of indexes of DAT files
    palette_files: Vec<&'a Palette>,

    /// List of indexes of PIC files
    pic_files: Vec<&'a Pic>,

    /// List of indexes of MAP files
    map_files: Vec<&'a Map>,
}

impl<'a> Extracted<'a> {

    pub fn new() -> Extracted<'a> {
        Extracted {
            files: HashMap::new(),
            text_files: Vec::new(),
            palette_files: Vec::new(),
            pic_files: Vec::new(),
            map_files: Vec::new(),
        }
    }

    pub fn insert_text(&mut self, text: Text) {
        let filename = text.filename.clone();

        let all = File::Text(&text);

        self.files.insert(filename, File::Text(text));
        self.text_files.push(&text);
    }

    pub fn insert_palette(&mut self, palette: Palette) {
        self.palette_files.push(&palette);
        self.files.insert(&palette.filename, File::Palette(palette));
    }

    pub fn insert_pic(&mut self, pic: Pic) {
        self.pic_files.push(&pic);
        self.files.insert(&pic.filename, File::Pic(pic));
    }

    pub fn insert_map(&mut self, map: Map) {
        self.map_files.push(&map);
        self.files.insert(&map.filename, File::Map(map));
    }



    pub fn get_palettes(&self) -> &[&Palette] {
        &self.palette_files
    }
}
*/

use super::bytes::Bytes;
use super::extracted::Extracted;
use super::file::*;

use std::collections::HashMap;

pub const ENCRYPTION_KEY: &[u8; 8] = b"32768GLB";

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Flag {
    Normal,
    Encrypted,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FatEntry {
    pub filename: String,
    pub flag: Flag,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileAllocationTable {
    pub entries: Vec<FatEntry>,
}


#[derive(Debug, PartialEq)]
pub struct GlbArchive {
    pub bytes: Vec<u8>
}


// The game only decrypts one FAT entry at a time, so after 28 bytes
// the key and "previous byte read" must be set back to the initial state.
const CHUNK_SIZE: usize = 28;


// If given first 28 bytes of file it parses them as header,
// that means that offset field is interpreted as number of files,
// which is value returned, and other fields are ignored.
fn parse_header(encrypted_bytes: &mut [u8]) -> usize {
    let entry = parse_fat_entry(encrypted_bytes);
    entry.offset as usize
}

// Parses single 28 byte File Allocation Table entry.
fn parse_fat_entry(encrypted_bytes: &mut [u8]) -> FatEntry {

    if encrypted_bytes.len() != CHUNK_SIZE {
        // TODO: Instead of panics we should return Result
        panic!("You should decrypt by chunks of 28 bytes.");
    }

    let mut decrypted_bytes = Bytes::from(encrypted_bytes);
    decrypted_bytes.decrypt(ENCRYPTION_KEY);

    let mut offset: usize = 0;

    let flag = match decrypted_bytes.read_u32(&mut offset) {
        0 => Flag::Normal,
        1 => Flag::Encrypted,
        other => panic!("Unknown flag {}", other)
    };

    let file_offset = decrypted_bytes.read_u32(&mut offset);

    let length = decrypted_bytes.read_u32(&mut offset);

    let filename = decrypted_bytes[12..28].split(|b| *b == 0).into_iter().next().unwrap();
    let filename = core::str::from_utf8(filename).unwrap().to_string();

    FatEntry { flag, offset: file_offset, length, filename }
}

impl GlbArchive {

    pub fn from_file(path: &str) -> Option<GlbArchive> {
        std::fs::read(path).ok().map(|bytes|
            GlbArchive { bytes }
        )
    }

    pub fn parse_fat(&mut self) -> FileAllocationTable {

        let mut offset: usize = 0;
        let fat_entries_count = parse_header(&mut self.bytes[offset..offset+CHUNK_SIZE]);
        let mut entries = Vec::with_capacity(fat_entries_count);

        for _ in 0..fat_entries_count {
            offset = offset + CHUNK_SIZE;
            let entry = parse_fat_entry(&mut self.bytes[offset..offset+CHUNK_SIZE]);
            entries.push(entry);
        }

        FileAllocationTable { entries }
    }

    pub fn extract_files(&mut self, fat: &FileAllocationTable) -> Extracted {

        let mut named_files: HashMap<String, File> = HashMap::with_capacity(fat.entries.len());

        let mut tiles: Vec<Pic> = Vec::new();

        let mut currently_reading_tiles = false;

        for entry in &fat.entries {
            let untyped_file = UntypedFile::read_file(self, entry);
            let filename = &untyped_file.filename;

            if filename.ends_with("TXT")
            {
                let text = untyped_file.get_txt();
                if let Some(t) = text {
                    named_files.insert(filename.to_owned(), File::Text(t));
                }
            }
            else if filename.ends_with("_DAT")
            {
                let palette = untyped_file.get_dat();
                if let Some(p) = palette {
                    named_files.insert(filename.to_owned(), File::Palette(p));
                }
            }
            else if filename.ends_with("_PIC")   ||
                    filename.ends_with("_PIC//") ||
                    filename.ends_with("_BLK")
            {
                let pic = untyped_file.get_pic();
                if let Some(p) = pic {
                    named_files.insert(filename.to_owned(), File::Pic(p));
                }
            }
            else if filename.ends_with("_MAP")
            {
                let map = untyped_file.get_map();
                if let Some(m) = map {
                    named_files.insert(filename.to_owned(), File::Map(m));
                }
            }
            else if filename.starts_with("STARTG")
            {
                currently_reading_tiles = true;
            }
            else if filename == "" && currently_reading_tiles
            {
                let pic = untyped_file.get_pic();
                if let Some(p) = pic {
                    tiles.push(p);
                }
            }
            else if filename.starts_with("ENDG")
            {
                currently_reading_tiles = false;
            }
            else
            {
                // println!("Can't parse unknown file {}!", filename);
            }
        }

        let tiles = Tiles { tiles };
        Extracted { named_files, tiles }
    }
}
mod utils;
mod untyped_file;
#[cfg(test)] mod test;

use utils::*;
use untyped_file::*;

use std::collections::HashMap;

const ENCRYPTION_KEY: &[u8; 8] = b"32768GLB";

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GlbArchive<'a> {
    bytes: &'a [u8]
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Flag {
    Normal,
    Encrypted,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FatEntry {
    pub filename: String,
    flag: Flag,
    offset: u32,
    length: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileAllocationTable {
    pub entries: Vec<FatEntry>,
}

impl<'a> GlbArchive<'a> {

    pub fn new(bytes: &'a [u8]) -> GlbArchive<'a> {
        GlbArchive { bytes }
    }

    pub fn parse_fat(&self) -> FileAllocationTable {

        // The game only decrypts one FAT entry at a time, so after 28 bytes
        // the key and "previous byte read" must be set back to the initial state.
        const CHUNK_SIZE: usize = 28;

        // Parses single 28 byte File Allocation Table entry.
        fn parse_fat_entry(encrypted_bytes: &[u8]) -> FatEntry {

            if encrypted_bytes.len() != CHUNK_SIZE {
                panic!("You should decrypt by chunks of 28 bytes.");
            }

            let decrypted_bytes = decrypt(encrypted_bytes, ENCRYPTION_KEY);

            let flag = match as_u32_le(&decrypted_bytes[0..4]) {
                0 => Flag::Normal,
                1 => Flag::Encrypted,
                other => panic!("Unknown flag {}", other)
            };

            let offset = as_u32_le(&decrypted_bytes[4..8]);

            let length = as_u32_le(&decrypted_bytes[8..12]);

            let filename = decrypted_bytes[12..28].split(|b| *b == 0).into_iter().next().unwrap();
            let filename = core::str::from_utf8(filename).unwrap().to_string();

            FatEntry { flag, offset, length, filename }
        }

        // If given first 28 bytes of file it parses them as header,
        // that means that offset field is interpreted as number of files,
        // which is value returned, and other fields are ignored.
        fn parse_header(encrypted_bytes: &[u8]) -> usize {
            let entry = parse_fat_entry(encrypted_bytes);
            entry.offset as usize
        }

        let mut offset: usize = 0;
        let fat_entries_count = parse_header(&self.bytes[offset..offset+CHUNK_SIZE]);
        let mut entries = Vec::with_capacity(fat_entries_count);

        for _ in 0..fat_entries_count {
            offset = offset + CHUNK_SIZE;
            let entry = parse_fat_entry(&self.bytes[offset..offset+CHUNK_SIZE]);
            entries.push(entry);
        }

        FileAllocationTable { entries }
    }

    pub fn extract_files(&self, fat: &FileAllocationTable) -> HashMap<String, File> {

        let mut file_map: HashMap<String, File> = HashMap::with_capacity(fat.entries.len());

        for entry in &fat.entries {
            let untyped_file = UntypedFile::read_file(self, entry);

            match untyped_file.filename {
                unknown => println!("Can't parse unknown file {}!", unknown)
            }
        }

        file_map
    }
}
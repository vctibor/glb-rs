#[derive(Debug)]
pub enum Flag {
    Normal,
    Encrypted
}

#[derive(Debug)]
pub struct FatEntry {
    pub flag: Flag,
    pub offset: u32,
    pub length: u32,
    pub filename: String
}

#[derive(Debug)]
pub struct FileAllocationTable {
    pub entries: Vec<FatEntry>,
}
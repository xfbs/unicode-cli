use std::io::Read;

pub const UNICODE_DATA: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/UnicodeData.txt"));

pub const UNICODE_BLOCKS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Blocks.txt"));

pub struct Block {
    start: char,
    end: char,
    name: String,
}

pub struct Blocks {
    blocks: Vec<Block>
}

impl Blocks {
    pub fn from_file<R: Read>(reader: R) -> Self {
        Blocks {
            blocks: vec![]
        }
    }
}

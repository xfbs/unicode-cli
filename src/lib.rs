use std::io::Read;
use std::cmp::Ordering;
use crate::data::UNICODE_BLOCKS;

mod data;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    start: u32,
    end: u32,
    name: &'static str,
}

impl Block {
    pub fn from(c: char) -> Option<Block> {
        let c = c as u32;
        let pos = UNICODE_BLOCKS.binary_search_by(|element|
            match (element.start <= c, element.end > c) {
                (true, true) => Ordering::Equal,
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => unreachable!()
            });

        pos.ok().map(|n| UNICODE_BLOCKS[n])
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

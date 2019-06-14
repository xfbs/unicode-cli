use crate::Block;

pub const UNICODE_BLOCKS: &[Block] = &include!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/blocks.rs"));

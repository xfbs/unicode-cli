use std::fmt;
use unic_ucd::*;
use regex::Regex;
use log::*;

pub struct Info {
    name: Name,
    category: GeneralCategory,
    block: &'static str,
    alphabetic: bool,
    bidi_mirrored: bool,
    case_ignorable: bool,
    cased: bool,
    lowercase: bool,
    uppercase: bool,
    whitespace: bool,
    age: Age,
}

impl Info {
    pub fn of(c: char) -> Option<Info> {
        Some(Info {
            name: Name::of(c)?,
            category: GeneralCategory::of(c),
            block: Block::of(c)?.name,
            alphabetic: is_alphabetic(c),
            bidi_mirrored: is_bidi_mirrored(c),
            case_ignorable: is_case_ignorable(c),
            cased: is_cased(c),
            lowercase: is_lowercase(c),
            uppercase: is_uppercase(c),
            whitespace: is_white_space(c),
            age: Age::of(c)?,
        })
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name:           {}\n", self.name)?;
        write!(f, "category:       {}\n", self.category)?;
        write!(f, "block:          {}\n", self.block)?;
        write!(f, "alphabetic:     {}\n", self.alphabetic)?;
        write!(f, "bidi_mirrored:  {}\n", self.bidi_mirrored)?;
        write!(f, "case_ignorable: {}\n", self.case_ignorable)?;
        write!(f, "cased:          {}\n", self.cased)?;
        write!(f, "lowercase:      {}\n", self.lowercase)?;
        write!(f, "uppercase:      {}\n", self.uppercase)?;
        write!(f, "whitespace:     {}\n", self.whitespace)?;
        write!(f, "alphabetic:     {}\n", self.alphabetic)?;
        write!(f, "since version:  {}\n", self.age.actual())
    }
}

pub fn parse_scalar_value(s: &str) -> Option<char> {
    let mut chars = s.chars();

    // if it's a single char, just return that.
    if let Some(c) = chars.next() {
        if chars.next() == None {
            return Some(c);
        }
    }

    // maybe it's a hex-encoded unicode char.
    if let Ok(raw) = u32::from_str_radix(s, 16) {
        if let Some(c) = std::char::from_u32(raw) {
            return Some(c);
        }
    }

    // maybe it's a character name
    if let Some(c) = unicode_names2::character(s) {
        return Some(c);
    }

    None
}

pub fn search(regex: &Regex) -> Vec<char> {
    let mut results = Vec::new();

    for block in BlockIter::new() {
        for candidate in block.range {
            if let Some(name) = Name::of(candidate) {
                let name = name.to_string();

                if regex.is_match(&name) {
                    results.push(candidate);
                }
            } else {
                debug!("no name for {} in {:?}", candidate as u32, block);
            }
        }
    }

    results
}

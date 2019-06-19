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

pub struct CharInfo {
    long: usize,
}

impl CharInfo {
    pub fn new(long: usize) -> Self {
        CharInfo {
            long
        }
    }

    pub fn display(&self, c: char) {
        let flags: [(char, fn(char) -> bool); 6] = [
            ('a', is_alphabetic),
            ('b', is_bidi_mirrored),
            ('c', is_cased),
            ('i', is_case_ignorable),
            ('u', is_uppercase),
            ('l', is_lowercase),
        ];

        if self.long >= 2 {
            print!("{}", chartype(c));
            for flag in &flags {
                if flag.1(c) {
                    print!("{}", flag.0);
                } else {
                    print!("-");
                }
            }

            print!(" {:6X} ", c as u32);
        }

        if self.long >= 1 {
            if let Some(block) = Block::of(c) {
                print!("{} ", block.name);
            } else {
                print!("Unknown Block ");
            }

            if let Some(name) = Name::of(c) {
                print!("{} ", name);
            } else {
                print!("None ");
            }

            println!("");
        }

        if self.long == 0 {
            print!("{}", c);
            /*
            use GeneralCategory::*;
            match GeneralCategory::of(c) {
                Control => println!("{}", c.escape_default()),
                SpaceSeparator => println!("'{}'", c.escape_default()),
                _ => println!("{}", c),
            }
            */
        }
    }
}

fn is_unicode(c: char) -> bool {
    Block::of(c).is_some()
}

fn chartype(c: char) -> char {
    if c.is_ascii() {
        'a'
    } else {
        'u'
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

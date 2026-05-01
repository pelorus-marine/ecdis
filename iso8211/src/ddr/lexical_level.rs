use std::{fmt, str::FromStr};

use crate::{Iso8211Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum LexicalLevel {
    Level0,
    Level1,
    Level2,
    Unknown,
}

impl fmt::Display for LexicalLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalLevel::Level0 => write!(f, "Level0"),
            LexicalLevel::Level1 => write!(f, "Level1"),
            LexicalLevel::Level2 => write!(f, "Level2"),
            LexicalLevel::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for LexicalLevel {
    type Err = Iso8211Error;

    fn from_str(value: &str) -> Result<LexicalLevel> {
        match value {
            "   " => Ok(LexicalLevel::Level0),
            "-A " => Ok(LexicalLevel::Level1),
            "%/@" => Ok(LexicalLevel::Level2),
            //FIXME: Find out what this lexical level is
            "%/G" => Ok(LexicalLevel::Unknown),
            e => Err(Iso8211Error::Parse(format!(
                "Invalid Truncated Escape Sequence: {}",
                e
            ))),
        }
    }
}

use std::fmt;

use crate::{Iso8211Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum Format {
    CharacterData(Option<usize>),
    UnsignedInteger(usize),
    SignedInteger(usize),
    SignedFloatingPoint,
    //SubFormat(Box<Format>),
}

impl Format {
    pub(super) fn read(value: &str) -> Result<Vec<Format>> {
        if value.is_empty() {
            return Err(Iso8211Error::Parse("empty format string".to_string()));
        }
        let formats: Vec<Format>;
        let mut multiplicity = 0;

        let mut chars = value.chars();
        loop {
            match chars.next() {
                Some('b') => {
                    if multiplicity == 0 {
                        multiplicity = 1;
                    }
                    break;
                }
                Some('A') => {
                    if multiplicity == 0 {
                        multiplicity = 1;
                    }
                    break;
                }
                Some(c) => {
                    if c.is_numeric() {
                        multiplicity = multiplicity * 10 + c.to_digit(10).unwrap() as usize;
                    } else {
                        return Err(Iso8211Error::Parse(format!(
                            "invalid format multiplicity: '{}'",
                            value
                        )));
                    }
                }
                None => return Err(Iso8211Error::Parse("empty format string".to_string())),
            }
        }

        let mut chars = value.chars();
        if multiplicity > 1 {
            let mstr = multiplicity.to_string();
            let mut len = mstr.chars();
            while len.next().is_some() {
                chars.next();
            }
        }
        match chars.next() {
            Some('b') => match chars.next() {
                Some('1') => match chars.next() {
                    Some('1') => formats = vec![Format::UnsignedInteger(1); multiplicity],
                    Some('2') => formats = vec![Format::UnsignedInteger(2); multiplicity],
                    Some('4') => formats = vec![Format::UnsignedInteger(4); multiplicity],
                    Some(_) => {
                        return Err(Iso8211Error::Parse(format!(
                            "invalid unsigned integer size: '{}'",
                            value
                        )));
                    }
                    None => {
                        return Err(Iso8211Error::Parse(format!(
                            "missing unsigned integer size: '{}'",
                            value
                        )));
                    }
                },
                Some('2') => match chars.next() {
                    Some('1') => formats = vec![Format::SignedInteger(1); multiplicity],
                    Some('2') => formats = vec![Format::SignedInteger(2); multiplicity],
                    Some('4') => formats = vec![Format::SignedInteger(4); multiplicity],
                    Some(_) => {
                        return Err(Iso8211Error::Parse(format!(
                            "invalid signed integer size: '{}'",
                            value
                        )));
                    }
                    None => {
                        return Err(Iso8211Error::Parse(format!(
                            "missing signed integer size: '{}'",
                            value
                        )));
                    }
                },
                Some('4') => match chars.next() {
                    Some('8') => formats = vec![Format::SignedFloatingPoint; multiplicity],
                    Some(_) => {
                        return Err(Iso8211Error::Parse(format!(
                            "invalid signed floating point size: '{}'",
                            value
                        )));
                    }
                    None => {
                        return Err(Iso8211Error::Parse(format!(
                            "missing signed floating point size: '{}'",
                            value
                        )));
                    }
                },
                Some(_) => {
                    return Err(Iso8211Error::Parse(format!(
                        "invalid signed floating point size: '{}'",
                        value
                    )));
                }
                None => {
                    return Err(Iso8211Error::Parse(format!(
                        "missing signed floating point size: '{}'",
                        value
                    )));
                }
            },
            Some('A') => match chars.next() {
                Some('(') => {
                    let mut size: usize = 0;
                    loop {
                        match chars.next() {
                            Some(')') => break,
                            Some(c) => {
                                if c.is_numeric() {
                                    size = size * 10 + c.to_digit(10).unwrap() as usize;
                                } else {
                                    return Err(Iso8211Error::Parse(format!(
                                        "invalid digit in signed floating point size: '{}'",
                                        value
                                    )));
                                }
                            }
                            None => {
                                return Err(Iso8211Error::Parse(format!(
                                    "invalid signed floating point size: '{}'",
                                    value
                                )));
                            }
                        }
                    }

                    formats = vec![Format::CharacterData(Some(size)); multiplicity]
                }
                Some(_) => {
                    return Err(Iso8211Error::Parse(format!(
                        "invalid character string format: '{}'",
                        value
                    )));
                }
                None => formats = vec![Format::CharacterData(None); multiplicity],
            },
            Some(_) => {
                return Err(Iso8211Error::Parse(format!(
                    "invalid data type in format: '{}'",
                    value
                )));
            }
            None => {
                return Err(Iso8211Error::Parse(format!(
                    "format missing data type: '{}'",
                    value
                )));
            }
        }

        if chars.next().is_some() {
            return Err(Iso8211Error::Parse(format!(
                "found an unexpected character after the format size: {}",
                value
            )));
        }

        Ok(formats)
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::CharacterData(o) => match o {
                None => write!(f, "CharacterData()"),
                Some(s) => write!(f, "CharacterData({})", s),
            },
            Format::UnsignedInteger(s) => write!(f, "UnsignedInteger({})", s),
            Format::SignedInteger(s) => write!(f, "SignedInteger({})", s),
            Format::SignedFloatingPoint => write!(f, "SignedFloatingPoint"),
        }
    }
}

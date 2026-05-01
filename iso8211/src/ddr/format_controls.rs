use std::str::FromStr;

use crate::{Iso8211Error, Result};

use super::Format;

#[derive(Debug)]
pub struct FormatControls {
    formats: Vec<Format>,
}

impl FormatControls {
    pub fn len(&self) -> usize {
        self.formats.len()
    }

    pub fn is_empty(&self) -> bool {
        self.formats.is_empty()
    }

    pub fn formats(&self) -> &[Format] {
        &self.formats
    }
}

impl FromStr for FormatControls {
    type Err = Iso8211Error;

    fn from_str(value: &str) -> Result<FormatControls> {
        let mut chars = value.chars();
        match chars.next() {
            Some('(') => match chars.last() {
                Some(')') => {
                    let mut formats: Vec<Format> = Vec::new();
                    let fs: String = value.chars().skip(1).take(value.len() - 2).collect();
                    let pieces = fs.split(',');
                    for piece in pieces {
                        match Format::read(piece) {
                            Ok(f) => formats.extend(f.iter().cloned()),
                            Err(e) => return Err(e),
                        }
                    }

                    Ok(FormatControls { formats })
                }
                Some(_) => Err(Iso8211Error::Parse(format!(
                    "invalid format controls string: '{}'",
                    value
                ))),
                None => Err(Iso8211Error::Parse(format!(
                    "invalid format controls string: '{}'",
                    value
                ))),
            },
            Some(_) => Err(Iso8211Error::Parse(format!(
                "invalid format controls string: '{}'",
                value
            ))),
            None => Err(Iso8211Error::Parse(
                "empty format controls string".to_string(),
            )),
        }
    }
}

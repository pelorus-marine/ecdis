use std::{fmt, str::FromStr};

use crate::{Iso8211Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    CharacterString = 0,
    ImplicitPoint = 1,
    ExplicitPoint = 2,
    Binary = 5,
    Mixed = 6,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::CharacterString => write!(f, "CharacterString"),
            DataType::ImplicitPoint => write!(f, "ImplicitPoint"),
            DataType::ExplicitPoint => write!(f, "ExplicitPoint"),
            DataType::Binary => write!(f, "Binary"),
            DataType::Mixed => write!(f, "Mixed"),
        }
    }
}

impl FromStr for DataType {
    type Err = Iso8211Error;

    fn from_str(value: &str) -> Result<DataType> {
        match value {
            "0" => Ok(DataType::CharacterString),
            "1" => Ok(DataType::ImplicitPoint),
            "2" => Ok(DataType::ExplicitPoint),
            "5" => Ok(DataType::Binary),
            "6" => Ok(DataType::Mixed),
            e => Err(Iso8211Error::Parse(format!(
                "Invalid Data Type Code: {}",
                e
            ))),
        }
    }
}

use std::{fmt, str::FromStr};

use crate::{Iso8211Error, Result};

#[derive(Debug, Clone, Copy)]
pub enum DataStructure {
    SingleDataItem,
    LinearStructure,
    MultiDimensionalStructure,
    Unknown,
}

impl fmt::Display for DataStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataStructure::SingleDataItem => write!(f, "SingleDataItem"),
            DataStructure::LinearStructure => write!(f, "LinearStructure"),
            DataStructure::MultiDimensionalStructure => write!(f, "MultiDimensionalStructure"),
            DataStructure::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for DataStructure {
    type Err = Iso8211Error;

    fn from_str(value: &str) -> Result<DataStructure> {
        match value {
            "0" => Ok(DataStructure::SingleDataItem),
            "1" => Ok(DataStructure::LinearStructure),
            "2" => Ok(DataStructure::MultiDimensionalStructure),
            "3" => Ok(DataStructure::Unknown),
            e => Err(Iso8211Error::Parse(format!(
                "Invalid Data Structure Code: {}",
                e
            ))),
        }
    }
}

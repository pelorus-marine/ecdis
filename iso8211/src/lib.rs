//! ISO 8211 (IHO transfer format) reader.
//!
//! This crate parses the **Data Descriptive Record** (schema) and **Data Records**
//! from ISO 8211 files, as used by IHO products such as S-57 ENC and S-101 datasets.
//!
//! # Example
//!
//! ```no_run
//! use iso8211::DataDescriptiveFile;
//!
//! let ddf = DataDescriptiveFile::read("path/to/file.000")?;
//! let _schema = ddf.data_descriptive_record();
//! let _rows = ddf.data_records();
//! # Ok::<(), iso8211::Iso8211Error>(())
//! ```
//!
//! See crate [`DataDescriptiveFile`] for the main entry point.

#![forbid(unsafe_code)]

pub mod ddr;

pub mod dr;

mod error;
pub use error::{Iso8211Error, Result};

mod data_descriptive_file;
pub use data_descriptive_file::DataDescriptiveFile;

mod directory;
use directory::Directory;

mod directory_entry;
use directory_entry::DirectoryEntry;

mod entry_map;
use entry_map::EntryMap;

mod leader;
use leader::Leader;

mod reader;
use reader::Reader;

/// binary value for ISO8211 field terminator
const FIELD_TERMINATOR: u8 = 0x1e;

/// binary value for ISO8211 unit terminator
const UNIT_TERMINATOR: u8 = 0x1f;

#[cfg(test)]
pub(crate) mod tests {
    pub fn to_bytes(value: &str) -> Vec<u8> {
        let value = value.replace(" ", "");
        (0..value.len())
            .step_by(2)
            .map(|i| value.get(i..i + 2).and_then(|sub| u8::from_str_radix(sub, 16).ok()).unwrap())
            .collect()
    }
}

use std::io::{BufReader, Cursor};
use std::path::Path;

use iso8211::DataDescriptiveFile;

use crate::decode::record_field_payload;
use crate::S101Error;

/// Loaded **S-101** ENC exchange: ISO 8211 container with structural validation only.
///
/// Feature-level decoding (geometry, attributes, portrayal) is intentionally **not**
/// implemented yet; this type gives a safe **load + probe** path for the **pelorus-ecdis**
/// integration crate and future decoders.
pub struct S101Dataset {
    inner: DataDescriptiveFile,
}

impl S101Dataset {
    /// Read an exchange file from disk (`.000`, etc.).
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, S101Error> {
        let inner = DataDescriptiveFile::read(path)?;
        validate_s101_structure(&inner)?;
        Ok(Self { inner })
    }

    /// Parse ENC exchange bytes (for example from a zip member or memory-mapped file).
    pub fn load_bytes(bytes: &[u8]) -> Result<Self, S101Error> {
        let inner = DataDescriptiveFile::read_buf(BufReader::new(Cursor::new(bytes)))?;
        validate_s101_structure(&inner)?;
        Ok(Self { inner })
    }

    /// Take ownership of an already-read [`DataDescriptiveFile`] after validation.
    pub fn from_iso8211_file(inner: DataDescriptiveFile) -> Result<Self, S101Error> {
        validate_s101_structure(&inner)?;
        Ok(Self { inner })
    }

    pub fn iso8211(&self) -> &DataDescriptiveFile {
        &self.inner
    }

    pub fn into_iso8211(self) -> DataDescriptiveFile {
        self.inner
    }

    pub fn record_count(&self) -> usize {
        self.inner.data_records().len()
    }

    /// Raw **DSID** field payload from the **first** data record, if present.
    ///
    /// This is the standard discovery record for S-100-family ENC; interpreting bytes as
    /// structured attributes comes later.
    pub fn first_record_dsid_payload(&self) -> Option<&[u8]> {
        record_field_payload(self.inner.data_records().first()?, "DSID")
    }
}

fn validate_s101_structure(ddf: &DataDescriptiveFile) -> Result<(), S101Error> {
    let ddr = ddf.data_descriptive_record();

    let has_dsid_ddf = ddr.data_descriptive_fields().iter().any(|f| {
        matches!(
            f.field_name(),
            // Older sample naming vs IHO long labels on exchange datasets.
            "DSID" | "Data Set Identification"
        )
    });

    if !has_dsid_ddf {
        return Err(S101Error::NotS101Dataset);
    }

    let records = ddf.data_records();
    if records.is_empty() {
        return Err(S101Error::MissingDataRecords);
    }

    let first = &records[0];
    let has_dsid = first
        .field_tags
        .iter()
        .zip(first.data_fields.iter())
        .any(|(tag, _)| tag == "DSID");

    if !has_dsid {
        return Err(S101Error::MissingDsidField);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn load_bytes_matches_load_when_fixture_present() {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../testdata/s101_sample.000");
        if !path.exists() {
            return;
        }
        let from_disk = S101Dataset::load(&path).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let from_mem = S101Dataset::load_bytes(&bytes).unwrap();
        assert_eq!(from_disk.record_count(), from_mem.record_count());
    }
}

use thiserror::Error;

/// Failure loading or validating an S-101 ENC exchange.
#[derive(Debug, Error)]
pub enum S101Error {
    #[error(transparent)]
    Iso8211(#[from] iso8211::Iso8211Error),

    #[error(
        "not an S-101-style dataset: DDR has no Data Descriptive Field for dataset identification (DSID / Data Set Identification)"
    )]
    NotS101Dataset,

    #[error("dataset has no data records")]
    MissingDataRecords,

    #[error("first data record has no DSID field (expected for S-101 discovery)")]
    MissingDsidField,
}

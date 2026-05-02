//! Fetch published **S-164** corpus archives over HTTPS.

use std::io::Read;
use std::time::Duration;

use crate::{S164Error, S164Result};

/// Browser-download URL for the **v1.2.0** prerelease bundle (`S-64_1.2.0.zip`) from
/// [iho-ohi/S-164-Sub-Group](https://github.com/iho-ohi/S-164-Sub-Group/releases).
///
/// Note: release artifacts are **prerelease**; pin the edition you certify against.
pub const DEFAULT_TEST_DATA_ZIP_V1_2_0_URL: &str =
    "https://github.com/iho-ohi/S-164-Sub-Group/releases/download/v1.2.0/S-64_1.2.0.zip";

/// Download arbitrary bytes (e.g. a zip corpus) with a bounded blocking timeout.
pub fn download_bytes(url: &str) -> S164Result<Vec<u8>> {
    download_bytes_with_timeout(url, Duration::from_secs(900))
}

/// Download with an explicit timeout (connect + overall response read).
pub fn download_bytes_with_timeout(url: &str, timeout: Duration) -> S164Result<Vec<u8>> {
    let resp = ureq::get(url)
        .timeout(timeout)
        .call()
        .map_err(|e| S164Error::Http(e.to_string()))?;

    let status = resp.status();
    if !(200..300).contains(&status) {
        return Err(S164Error::HttpStatus {
            url: url.to_string(),
            status,
        });
    }

    let mut buf = Vec::new();
    resp.into_reader().read_to_end(&mut buf)?;
    Ok(buf)
}

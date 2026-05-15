//! Fetch published **S-164** corpus archives over HTTPS, with on-disk caching and SHA-256 verification.

use std::path::Path;
use std::time::Duration;

use crate::cache::{cache_path_for_url, write_atomic};
use crate::verify::verify_sha256;
use crate::{S164Error, S164Result};

/// Browser-download URL for the **v1.2.0** prerelease bundle (`S-64_1.2.0.zip`) from
/// [iho-ohi/S-164-Sub-Group](https://github.com/iho-ohi/S-164-Sub-Group/releases).
///
/// Note: release artifacts are **prerelease**; pin the edition you certify against.
pub const DEFAULT_TEST_DATA_ZIP_V1_2_0_URL: &str =
    "https://github.com/iho-ohi/S-164-Sub-Group/releases/download/v1.2.0/S-64_1.2.0.zip";

/// SHA-256 (lowercase hex) of the `S-64_1.2.0.zip` asset at
/// [`DEFAULT_TEST_DATA_ZIP_V1_2_0_URL`]. Update only when intentionally rolling the edition.
pub const DEFAULT_TEST_DATA_ZIP_V1_2_0_SHA256: &str =
    "a3e0cab1d3484e4c8859bfc48ec7b982c3b1aefa4e97b090415b8a2599758f5e";

/// Download arbitrary bytes (e.g. a zip corpus) with a bounded blocking timeout.
pub fn download_bytes(url: &str) -> S164Result<Vec<u8>> {
    download_bytes_with_timeout(url, Duration::from_secs(900))
}

/// Download with an explicit timeout (connect + overall response read).
pub fn download_bytes_with_timeout(url: &str, timeout: Duration) -> S164Result<Vec<u8>> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .build()
        .into();

    let mut resp = agent
        .get(url)
        .call()
        .map_err(|e| S164Error::Http(e.to_string()))?;

    let status = resp.status().as_u16();
    if !(200..300).contains(&status) {
        return Err(S164Error::HttpStatus {
            url: url.to_string(),
            status,
        });
    }

    let buf = resp
        .body_mut()
        .read_to_vec()
        .map_err(|e| S164Error::Http(e.to_string()))?;
    Ok(buf)
}

/// Return bytes for `url`, using the on-disk cache when present and valid.
///
/// Behavior:
/// 1. Resolve cache path (see [`crate::cache::cache_path_for_url`]).
/// 2. If a cached file exists and `expected_sha256` is `None` or matches, return its bytes.
/// 3. Otherwise download, verify (when `expected_sha256.is_some()`), and atomically
///    persist into the cache before returning.
pub fn cached_download(url: &str, expected_sha256: Option<&str>) -> S164Result<Vec<u8>> {
    let cache_path = cache_path_for_url(url)?;
    if let Some(bytes) = try_read_valid_cache(&cache_path, expected_sha256)? {
        return Ok(bytes);
    }
    let bytes = download_bytes(url)?;
    if let Some(expected) = expected_sha256 {
        verify_sha256(&bytes, expected, url)?;
    }
    write_atomic(&cache_path, &bytes)?;
    Ok(bytes)
}

fn try_read_valid_cache(
    cache_path: &Path,
    expected_sha256: Option<&str>,
) -> S164Result<Option<Vec<u8>>> {
    if !cache_path.exists() {
        return Ok(None);
    }
    let bytes = std::fs::read(cache_path)?;
    if let Some(expected) = expected_sha256
        && verify_sha256(&bytes, expected, &cache_path.display().to_string()).is_err()
    {
        return Ok(None);
    }
    Ok(Some(bytes))
}

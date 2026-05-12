//! On-disk cache for downloaded corpus archives.
//!
//! Default location: [`dirs::cache_dir`]`()/pelorus-marine/s-164/`.
//! Override the whole directory with the **`S164_CACHE_DIR`** environment variable.

use std::path::{Path, PathBuf};

use crate::{S164Error, S164Result};

/// Environment variable that overrides the default cache directory.
pub const CACHE_DIR_ENV: &str = "S164_CACHE_DIR";

/// Resolve the cache root directory. Creates it on disk if it doesn't exist.
pub fn cache_root() -> S164Result<PathBuf> {
    let root = if let Some(custom) = std::env::var_os(CACHE_DIR_ENV) {
        PathBuf::from(custom)
    } else {
        let base = dirs::cache_dir().ok_or(S164Error::CacheDirUnavailable)?;
        base.join("pelorus-marine").join("s-164")
    };
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

/// Pick the cache filename from a download URL (final non-empty path segment).
pub fn cache_filename_from_url(url: &str) -> S164Result<String> {
    let scheme_end = url.find("://").map(|p| p + 3).unwrap_or(0);
    let after_scheme = &url[scheme_end..];
    let path_start = after_scheme
        .find('/')
        .map(|p| p + 1)
        .ok_or_else(|| S164Error::CacheFilenameFromUrl(url.to_string()))?;
    let path = after_scheme[path_start..]
        .split(['?', '#'])
        .next()
        .unwrap_or("");
    let last = path
        .rsplit('/')
        .find(|seg| !seg.is_empty())
        .ok_or_else(|| S164Error::CacheFilenameFromUrl(url.to_string()))?;
    if last.starts_with('.') {
        return Err(S164Error::CacheFilenameFromUrl(url.to_string()));
    }
    Ok(last.to_string())
}

/// Full cache path for a given URL (does not create the file).
pub fn cache_path_for_url(url: &str) -> S164Result<PathBuf> {
    Ok(cache_root()?.join(cache_filename_from_url(url)?))
}

/// Atomically write `bytes` to `final_path` via a temp sibling + rename.
pub fn write_atomic(final_path: &Path, bytes: &[u8]) -> S164Result<()> {
    let parent = final_path
        .parent()
        .ok_or(S164Error::CacheDirUnavailable)?;
    std::fs::create_dir_all(parent)?;
    let mut tmp = final_path.as_os_str().to_owned();
    tmp.push(".partial");
    let tmp_path = PathBuf::from(tmp);
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, final_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_from_github_release_url() {
        assert_eq!(
            cache_filename_from_url(
                "https://github.com/iho-ohi/S-164-Sub-Group/releases/download/v1.2.0/S-64_1.2.0.zip"
            )
            .unwrap(),
            "S-64_1.2.0.zip"
        );
    }

    #[test]
    fn filename_strips_query_and_fragment() {
        assert_eq!(
            cache_filename_from_url("https://example.com/a/b/file.zip?token=x").unwrap(),
            "file.zip"
        );
    }

    #[test]
    fn rejects_url_without_filename() {
        assert!(cache_filename_from_url("https://example.com/").is_err());
    }
}

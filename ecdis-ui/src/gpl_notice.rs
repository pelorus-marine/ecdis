//! GPL-3.0 distribution notice for the shipped `ecdis-ui` binary (Path A).

const DEFAULT_SOURCE_OFFER: &str = "file:///usr/share/doc/ecdis-ui/source-offer.txt";

/// Human-readable notice shown in the in-app About / License panel.
pub fn license_notice_text() -> String {
    let ui_version = env!("ECDIS_UI_VERSION");
    let slint_version = env!("SLINT_VERSION");
    let git_rev = option_env!("ECDIS_UI_GIT_REV").unwrap_or("(unknown)");
    let source = source_offer_uri();

    format!(
        "Pelorus ECDIS UI (ecdis-ui) — distributed under GNU GPL version 3\n\
         \n\
         This program incorporates the Slint UI toolkit (https://slint.dev), used under GPL-3.0-only.\n\
         \n\
         Build: ecdis-ui {ui_version} (git {git_rev})\n\
         Slint: {slint_version}\n\
         \n\
         Corresponding source for this binary (including complete buildable source tree and \
         Cargo.lock) must be offered under the same GPL-3.0 terms when you distribute this program.\n\
         \n\
         Source offer: {source}\n\
         On-device copy: /usr/share/doc/ecdis-ui/\n\
         \n\
         Third-party crate licenses: /usr/share/doc/ecdis-ui/THIRD_PARTY_NOTICES\n\
         Full GPL-3.0 text: /usr/share/doc/ecdis-ui/GPL-3.0-only.txt\n\
         \n\
         Rust sources in the Pelorus ecdis repository are also available under MIT OR Apache-2.0 \
         for library use; see DISTRIBUTION.md in the ecdis-ui crate."
    )
}

/// URI or path to the source offer (env override for images and CI).
pub fn source_offer_uri() -> String {
    std::env::var("ECDIS_SOURCE_OFFER_URI").unwrap_or_else(|_| DEFAULT_SOURCE_OFFER.to_string())
}

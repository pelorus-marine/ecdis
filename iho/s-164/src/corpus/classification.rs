/// How the IHO test corpus uses an exchange set, derived from its directory prefix.
///
/// The IHO conformance manual scopes negative tests by directory naming; this enum
/// captures the layer at which a negative test is designed to fail. Positive cases
/// are the default for any prefix not listed below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Classification {
    /// Standard dataset, expected to load cleanly through every layer.
    Positive,
    /// Byte-level malformed dataset (e.g. `CorruptData/…`). ISO 8211 parsers must reject.
    NegativeBytes,
    /// Bytes parse, but S-101 update-sequence rules are violated (e.g. `InvalidSequence00N/…`).
    NegativeUpdateSequence,
    /// Recognised exchange set whose role is not a parse-failure scenario.
    Other,
}

impl Classification {
    /// Classify by exchange-set prefix (path inside the zip ending in `/`).
    #[must_use]
    pub fn from_exchange_set_prefix(prefix: &str) -> Self {
        if prefix.contains("CorruptData") {
            Self::NegativeBytes
        } else if prefix.contains("InvalidSequence") {
            Self::NegativeUpdateSequence
        } else {
            Self::Positive
        }
    }

    /// `true` when the corpus author expects parsing to fail at the ISO 8211 layer.
    #[must_use]
    pub fn expects_iso8211_parse_failure(self) -> bool {
        matches!(self, Self::NegativeBytes)
    }
}

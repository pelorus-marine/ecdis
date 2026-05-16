//! High-level corpus abstraction over an S-164 zip bundle.
//!
//! [`Corpus::fetch_default`] downloads, caches, and verifies the IHO **v1.2.0** asset.
//! [`Corpus::open`] / [`Corpus::from_bytes`] cover the offline cases. Once constructed,
//! the corpus exposes precomputed [`ExchangeSetEntry`] / [`DatasetEntry`] indexes and
//! reads raw dataset bytes from the archive on demand.
//!
//! Each public type lives in its own file under `corpus/`; this `mod.rs` is just
//! the namespace assembly point.

mod build;
mod catalogue_entry;
mod classification;
mod corpus;
mod dataset_entry;
mod exchange_set_entry;

pub use catalogue_entry::CatalogueEntry;
pub use classification::Classification;
pub use corpus::Corpus;
pub use dataset_entry::DatasetEntry;
pub use exchange_set_entry::ExchangeSetEntry;

#[cfg(test)]
mod tests {
    use super::Classification;

    #[test]
    fn classification_known_prefixes() {
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/CorruptData/"),
            Classification::NegativeBytes
        );
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/InvalidSequence001/"),
            Classification::NegativeUpdateSequence
        );
        assert_eq!(
            Classification::from_exchange_set_prefix("S-100/DisplayStandard/"),
            Classification::Positive
        );
    }

    #[test]
    fn only_negative_bytes_expects_iso8211_failure() {
        assert!(Classification::NegativeBytes.expects_iso8211_parse_failure());
        assert!(!Classification::NegativeUpdateSequence.expects_iso8211_parse_failure());
        assert!(!Classification::Positive.expects_iso8211_parse_failure());
        assert!(!Classification::Other.expects_iso8211_parse_failure());
    }
}

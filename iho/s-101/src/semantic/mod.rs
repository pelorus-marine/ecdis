//! Minimal **semantic inventory** over ISO 8211 data records — groundwork for FC-driven decode.
//!
//! S-101 features are carried in data records that include an **`FRID`** directory field.
//! Full FC-backed decode lives in [`crate::graph`] and [`crate::record`]; this module keeps
//! cheap iterators for stage-1 portrayal and dashboards.
//!
//! Each public type lives in its own file under `semantic/`; this `mod.rs` is just
//! the namespace assembly point.

mod feature_catalogue_pin;
mod feature_inventory_summary;
mod raw_feature_record_ref;

pub use feature_catalogue_pin::FeatureCataloguePin;
pub use feature_inventory_summary::FeatureInventorySummary;
pub use raw_feature_record_ref::RawFeatureRecordRef;

use crate::S101Dataset;
use crate::decode::record_field_payload;

impl S101Dataset {
    /// Iterate data records that expose an **`FRID`** tag (candidate feature records).
    pub fn iter_raw_feature_records(&self) -> impl Iterator<Item = RawFeatureRecordRef<'_>> + '_ {
        self.iso8211()
            .data_records()
            .iter()
            .enumerate()
            .filter(|(_, rec)| rec.field_tags.iter().any(|t| t == "FRID"))
            .map(|(i, rec)| RawFeatureRecordRef {
                record_index: i,
                frid_payload: record_field_payload(rec, "FRID"),
            })
    }

    /// Count candidate feature records (`FRID`-bearing rows).
    pub fn feature_record_count(&self) -> usize {
        self.iter_raw_feature_records().count()
    }

    /// Cheap semantic probe for dashboards / orchestration (no FC XML required).
    pub fn feature_inventory_summary(&self) -> FeatureInventorySummary {
        FeatureInventorySummary {
            records_with_frid: self.feature_record_count(),
            total_data_records: self.record_count(),
        }
    }

    /// Inventory tied to a catalogue pin (edition strings only — FC XML validation TODO).
    pub fn feature_inventory_with_pin(&self, pin: &FeatureCataloguePin) -> FeatureInventorySummary {
        let _ = pin;
        self.feature_inventory_summary()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::S101Dataset;

    #[test]
    fn feature_inventory_matches_iterator_when_fixture_present() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../testdata/s101_sample.000");
        if !path.exists() {
            return;
        }
        let dataset = S101Dataset::load(path).unwrap();
        let inv = dataset.feature_inventory_summary();
        assert_eq!(inv.records_with_frid, dataset.feature_record_count());
        assert_eq!(inv.total_data_records, dataset.record_count());
    }
}

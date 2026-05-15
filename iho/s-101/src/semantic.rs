//! Minimal **semantic inventory** over ISO 8211 data records — groundwork for FC-driven decode.
//!
//! S-101 features are carried in data records that include an **`FRID`** directory field.
//! Full FC-backed decode lives in [`crate::graph`] and [`crate::record`]; this module keeps
//! cheap iterators for stage-1 portrayal and dashboards.

use crate::S101Dataset;
use crate::decode::record_field_payload;

/// Summary counts derived without yet parsing FC-backed attributes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FeatureInventorySummary {
    /// Data records containing an **`FRID`** field (feature records).
    pub records_with_frid: usize,
    /// Total ISO 8211 data records (includes discovery / structure rows).
    pub total_data_records: usize,
}

/// Reference to one raw feature-shaped row (still bytes — FC assigns meaning).
#[derive(Debug, Clone, Copy)]
pub struct RawFeatureRecordRef<'a> {
    pub record_index: usize,
    pub frid_payload: Option<&'a [u8]>,
}

/// Feature catalogue edition pin used **together with** [`crate::edition`] constants.
///
/// FC XML ingestion lives outside this first slice; applications should set these from DSID + FC file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureCataloguePin {
    pub product_specification_edition: String,
    pub feature_catalogue_edition: String,
}

impl Default for FeatureCataloguePin {
    fn default() -> Self {
        Self {
            product_specification_edition: crate::edition::TARGET_PRODUCT_SPECIFICATION_EDITION
                .to_string(),
            feature_catalogue_edition: String::new(),
        }
    }
}

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

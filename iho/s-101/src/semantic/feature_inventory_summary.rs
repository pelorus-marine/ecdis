/// Summary counts derived without yet parsing FC-backed attributes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FeatureInventorySummary {
    /// Data records containing an **`FRID`** field (feature records).
    pub records_with_frid: usize,
    /// Total ISO 8211 data records (includes discovery / structure rows).
    pub total_data_records: usize,
}

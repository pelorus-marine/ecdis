/// One **S100_DatasetDiscoveryMetadata** block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetDiscovery {
    pub file_uri: String,
    pub product_identifier: Option<String>,
}

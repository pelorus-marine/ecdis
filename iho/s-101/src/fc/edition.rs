//! Feature catalogue **edition** probe (product id + version triple).

/// Summary extracted from `S100FC:S100_FC_FeatureCatalogue` XML (namespaced tags).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FcEditionSummary {
    pub product_id: String,
    pub version_number: String,
    pub version_date: String,
}

fn between_prefixed(xml: &str, local: &str) -> Option<String> {
    let open = format!("<S100FC:{local}>");
    let close = format!("</S100FC:{local}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(xml[start..end].trim().to_string())
}

/// Parse edition triple from **full FC XML** bytes (IHO `S100_FC_FeatureCatalogue` export).
pub fn parse_fc_edition_summary(
    xml_bytes: &[u8],
) -> Result<FcEditionSummary, super::FcCatalogParseError> {
    let xml = std::str::from_utf8(xml_bytes).map_err(super::FcCatalogParseError::Utf8)?;
    Ok(FcEditionSummary {
        product_id: between_prefixed(xml, "productId")
            .ok_or(super::FcCatalogParseError::MissingProductId)?,
        version_number: between_prefixed(xml, "versionNumber")
            .ok_or(super::FcCatalogParseError::MissingVersionNumber)?,
        version_date: between_prefixed(xml, "versionDate")
            .ok_or(super::FcCatalogParseError::MissingVersionDate)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_initial_catalogue_snippet() {
        let xml = br#"<?xml version="1.0"?>
<S100FC:S100_FC_FeatureCatalogue xmlns:S100FC="http://www.iho.int/S100FC/5.0">
  <S100FC:productId>S-101</S100FC:productId>
  <S100FC:versionNumber>1.0.2</S100FC:versionNumber>
  <S100FC:versionDate>2022-02-28</S100FC:versionDate>
</S100FC:S100_FC_FeatureCatalogue>"#;
        let s = parse_fc_edition_summary(xml).unwrap();
        assert_eq!(s.product_id, "S-101");
        assert_eq!(s.version_number, "1.0.2");
        assert_eq!(s.version_date, "2022-02-28");
    }
}

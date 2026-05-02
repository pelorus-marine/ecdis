//! Minimal **Feature Catalogue XML** probe — edition metadata only (full FC model is future work).

/// Summary extracted from `S100FC:S100_FC_FeatureCatalogue` XML (namespaced tags).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FcEditionSummary {
    pub product_id: String,
    pub version_number: String,
    pub version_date: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FcCatalogParseError {
    Utf8(std::str::Utf8Error),
    MissingProductId,
    MissingVersionNumber,
    MissingVersionDate,
}

impl std::fmt::Display for FcCatalogParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8(e) => write!(f, "invalid UTF-8 in FC XML: {e}"),
            Self::MissingProductId => write!(f, "missing S100FC:productId element"),
            Self::MissingVersionNumber => write!(f, "missing S100FC:versionNumber element"),
            Self::MissingVersionDate => write!(f, "missing S100FC:versionDate element"),
        }
    }
}

impl std::error::Error for FcCatalogParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

fn between_prefixed(xml: &str, local: &str) -> Option<String> {
    let open = format!("<S100FC:{local}>");
    let close = format!("</S100FC:{local}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(xml[start..end].trim().to_string())
}

/// Parse edition triple from **full FC XML** bytes (IHO `S100_FC_FeatureCatalogue` export).
pub fn parse_fc_edition_summary(xml_bytes: &[u8]) -> Result<FcEditionSummary, FcCatalogParseError> {
    let xml = std::str::from_utf8(xml_bytes).map_err(FcCatalogParseError::Utf8)?;
    Ok(FcEditionSummary {
        product_id: between_prefixed(xml, "productId")
            .ok_or(FcCatalogParseError::MissingProductId)?,
        version_number: between_prefixed(xml, "versionNumber")
            .ok_or(FcCatalogParseError::MissingVersionNumber)?,
        version_date: between_prefixed(xml, "versionDate")
            .ok_or(FcCatalogParseError::MissingVersionDate)?,
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

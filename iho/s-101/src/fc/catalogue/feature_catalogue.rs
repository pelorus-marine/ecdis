use std::collections::HashMap;

use roxmltree::Node;

use crate::fc::FcCatalogParseError;
use crate::fc::edition::{FcEditionSummary, parse_fc_edition_summary};

use super::{ComplexAttribute, FeatureType, InformationType, ListedValue, SimpleAttribute};

/// Parsed **S-101** feature catalogue (enough for attribute + feature-class resolution).
#[derive(Debug, Clone)]
pub struct FeatureCatalogue {
    pub edition: FcEditionSummary,
    pub simple_attributes: Vec<SimpleAttribute>,
    pub complex_attributes: Vec<ComplexAttribute>,
    pub feature_types: Vec<FeatureType>,
    pub information_types: Vec<InformationType>,
    by_source_id: HashMap<u32, usize>,
    by_feature_code: HashMap<String, usize>,
}

impl FeatureCatalogue {
    /// Parse full FC XML bytes.
    pub fn parse_xml(xml_bytes: &[u8]) -> Result<Self, FcCatalogParseError> {
        let edition = parse_fc_edition_summary(xml_bytes)?;
        let xml = std::str::from_utf8(xml_bytes).map_err(FcCatalogParseError::Utf8)?;
        let doc =
            roxmltree::Document::parse(xml).map_err(|e| FcCatalogParseError::Xml(e.to_string()))?;

        let mut simple_attributes = Vec::new();
        let mut by_source_id = HashMap::new();

        for n in doc.descendants() {
            if !is_element_named(n, "S100_FC_SimpleAttribute") {
                continue;
            }
            if let Some(sa) = parse_simple_attribute(n) {
                if let Some(sid) = sa.source_identifier {
                    by_source_id.insert(sid, simple_attributes.len());
                }
                simple_attributes.push(sa);
            }
        }

        let mut complex_attributes = Vec::new();
        for n in doc.descendants() {
            if !is_element_named(n, "S100_FC_ComplexAttribute") {
                continue;
            }
            if let Some(code) = child_text(n, "code") {
                complex_attributes.push(ComplexAttribute { code });
            }
        }

        let mut feature_types = Vec::new();
        let mut by_feature_code = HashMap::new();
        for n in doc.descendants() {
            if !is_element_named(n, "S100_FC_FeatureType") {
                continue;
            }
            if let Some(ft) = parse_feature_type(n) {
                by_feature_code.insert(ft.code.clone(), feature_types.len());
                feature_types.push(ft);
            }
        }

        let mut information_types = Vec::new();
        for n in doc.descendants() {
            if !is_element_named(n, "S100_FC_InformationType") {
                continue;
            }
            if let Some(code) = child_text(n, "code") {
                information_types.push(InformationType { code });
            }
        }

        Ok(Self {
            edition,
            simple_attributes,
            complex_attributes,
            feature_types,
            information_types,
            by_source_id,
            by_feature_code,
        })
    }

    #[must_use]
    pub fn simple_attr_by_source_id(&self, id: u16) -> Option<&SimpleAttribute> {
        self.by_source_id
            .get(&u32::from(id))
            .and_then(|&i| self.simple_attributes.get(i))
    }

    #[must_use]
    pub fn simple_attr_by_atix(&self, atix: u16) -> Option<&SimpleAttribute> {
        self.simple_attr_by_source_id(atix)
    }

    #[must_use]
    pub fn feature_type_by_code(&self, code: &str) -> Option<&FeatureType> {
        self.by_feature_code.get(code).and_then(|&i| self.feature_types.get(i))
    }
}

fn is_element_named(n: Node<'_, '_>, name: &str) -> bool {
    n.is_element() && n.tag_name().name() == name
}

fn child_text(parent: Node<'_, '_>, local: &str) -> Option<String> {
    for c in parent.children() {
        if c.is_element() && c.tag_name().name() == local {
            return Some(c.text()?.trim().to_string());
        }
    }
    None
}

fn child_node<'a, 'input>(parent: Node<'a, 'input>, local: &str) -> Option<Node<'a, 'input>> {
    parent.children().find(|c| c.is_element() && c.tag_name().name() == local)
}

fn parse_simple_attribute(n: Node<'_, '_>) -> Option<SimpleAttribute> {
    let code = child_text(n, "code")?;
    let alias = child_text(n, "alias");
    let value_type = child_text(n, "valueType").unwrap_or_default();
    let mut source_identifier = None;
    if let Some(dr) = child_node(n, "definitionReference")
        && let Some(s) = child_text(dr, "sourceIdentifier")
    {
        source_identifier = s.parse().ok();
    }
    let mut listed_values = Vec::new();
    if let Some(lv_root) = child_node(n, "listedValues") {
        for lv in lv_root.children() {
            if !is_element_named(lv, "listedValue") {
                continue;
            }
            let label = child_text(lv, "label").unwrap_or_default();
            let code_s = child_text(lv, "code")?;
            let code = code_s.parse().ok()?;
            listed_values.push(ListedValue { code, label });
        }
    }
    Some(SimpleAttribute {
        code,
        alias,
        value_type,
        source_identifier,
        listed_values,
    })
}

fn parse_feature_type(n: Node<'_, '_>) -> Option<FeatureType> {
    let code = child_text(n, "code")?;
    let alias = child_text(n, "alias");
    let mut permitted_primitives = Vec::new();
    for c in n.children() {
        if c.is_element()
            && c.tag_name().name() == "permittedPrimitives"
            && let Some(t) = c.text()
        {
            permitted_primitives.push(t.trim().to_string());
        }
    }
    let mut attribute_refs = Vec::new();
    for c in n.children() {
        if !c.is_element() || c.tag_name().name() != "attributeBinding" {
            continue;
        }
        for ab in c.children() {
            if ab.is_element()
                && ab.tag_name().name() == "attribute"
                && let Some(r) = ab.attribute("ref")
            {
                attribute_refs.push(r.to_string());
            }
        }
    }
    Some(FeatureType {
        code,
        alias,
        permitted_primitives,
        attribute_refs,
    })
}

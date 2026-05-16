use crate::fc::SimpleAttribute;

use super::AttributeValue;

#[derive(Debug)]
pub struct ResolvedAttribute<'fc> {
    pub fc_entry: &'fc SimpleAttribute,
    pub value: AttributeValue,
}

//! Look up payloads by **ISO 8211 directory tag** (four-character field name).

use iso8211::dr::DataRecord;

/// Return the payload for the first field with tag `tag` in `record`.
pub fn record_field_payload<'a>(record: &'a DataRecord, tag: &str) -> Option<&'a [u8]> {
    record
        .field_tags
        .iter()
        .zip(record.data_fields.iter())
        .find(|(t, _)| *t == tag)
        .map(|(_, f)| f.user_data())
}

/// Return the payload for the `n`th occurrence of `tag` (0 = first), if any.
pub fn field_payload<'a>(record: &'a DataRecord, tag: &str, occurrence: usize) -> Option<&'a [u8]> {
    let mut seen = 0usize;
    for (t, f) in record.field_tags.iter().zip(record.data_fields.iter()) {
        if t == tag {
            if seen == occurrence {
                return Some(f.user_data());
            }
            seen += 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use iso8211::dr::{DataField, DataRecord};

    use super::field_payload;

    #[test]
    fn find_second_occurrence() {
        let record = DataRecord {
            field_tags: vec!["FOO".into(), "BAR".into(), "FOO".into()],
            data_fields: vec![
                DataField::from_vec(vec![1]),
                DataField::from_vec(vec![2]),
                DataField::from_vec(vec![3]),
            ],
        };
        assert_eq!(field_payload(&record, "FOO", 0), Some(&[1u8][..]));
        assert_eq!(field_payload(&record, "FOO", 1), Some(&[3u8][..]));
        assert_eq!(field_payload(&record, "FOO", 2), None);
    }
}

//! Typed **S-101** ISO 8211 data records (first decode slice for S-64 v1.2.0 layout).

mod attribute;
mod composite;
mod crs;
mod cuco_ref;
mod curve;
mod dataset;
mod feature;
mod foid;
mod frid;
mod identifier;
mod mrid;
mod point;
mod ptas_ref;
mod rias_ref;
mod spatial_ref;
mod surface;

pub use attribute::{RawAttributeTuple, parse_attr_tuples};
pub use composite::CompositeCurveRecord;
pub use crs::CoordinateSystemRecord;
pub use cuco_ref::CucoRef;
pub use curve::CurveRecord;
pub use dataset::DatasetDescriptionRecord;
pub use feature::FeatureRecord;
pub use foid::parse_foid;
pub use frid::FridHeader;
pub use identifier::RecordIdentifier;
pub use mrid::MridRecord;
pub use point::PointRecord;
pub use spatial_ref::SpasRef;
pub use rias_ref::RiasRef;
pub use surface::SurfaceRecord;

use iso8211::dr::DataRecord;

/// Classified S-101 data record.
#[derive(Debug, Clone)]
pub enum Record {
    DatasetDescription(DatasetDescriptionRecord),
    CoordinateSystem(CoordinateSystemRecord),
    Point(PointRecord),
    Mrid(MridRecord),
    Curve(CurveRecord),
    CompositeCurve(CompositeCurveRecord),
    Surface(SurfaceRecord),
    Feature(FeatureRecord),
    Unknown {
        record_index: usize,
        tags: Vec<String>,
    },
}

fn record_field_pairs(rec: &DataRecord) -> Vec<(String, Vec<u8>)> {
    rec.field_tags
        .iter()
        .zip(rec.data_fields.iter())
        .map(|(t, df)| (t.clone(), df.user_data().to_vec()))
        .collect()
}

fn tags_vec(rec: &DataRecord) -> Vec<String> {
    rec.field_tags.clone()
}

/// Classify a single ISO 8211 [`DataRecord`] into a [`Record`].
#[must_use]
#[allow(clippy::collapsible_if)] // many independent tag probes; nested reads stay obvious
pub fn classify_record(record_index: usize, rec: &DataRecord) -> Record {
    let tags = tags_vec(rec);
    if tags.iter().any(|t| t == "FRID") {
        if let Some(f) = FeatureRecord::parse(record_index, rec) {
            return Record::Feature(f);
        }
    }
    if tags.iter().any(|t| t == "DSID") {
        let pairs = record_field_pairs(rec);
        if let Some(d) = DatasetDescriptionRecord::parse(record_index, &pairs) {
            return Record::DatasetDescription(d);
        }
    }
    if tags.iter().any(|t| t == "CSID") {
        let pairs = record_field_pairs(rec);
        if let Some(c) = CoordinateSystemRecord::parse(record_index, &pairs) {
            return Record::CoordinateSystem(c);
        }
    }
    if tags.iter().any(|t| t == "CCID") {
        let cr = crate::decode::record_field_payload(rec, "CCID");
        let cucos: Vec<&[u8]> = rec
            .field_tags
            .iter()
            .zip(rec.data_fields.iter())
            .filter(|(t, _)| *t == "CUCO")
            .map(|(_, df)| df.user_data())
            .collect();
        if let Some(cc) = cr {
            if let Some(ccr) = CompositeCurveRecord::parse(record_index, cc, &cucos) {
                return Record::CompositeCurve(ccr);
            }
        }
    }
    if tags.iter().any(|t| t == "SRID") {
        let sr = crate::decode::record_field_payload(rec, "SRID");
        let rias: Vec<&[u8]> = rec
            .field_tags
            .iter()
            .zip(rec.data_fields.iter())
            .filter(|(t, _)| *t == "RIAS")
            .map(|(_, df)| df.user_data())
            .collect();
        if let Some(s) = sr {
            if let Some(sur) = SurfaceRecord::parse(record_index, s, &rias) {
                return Record::Surface(sur);
            }
        }
    }
    if tags.iter().any(|t| t == "CRID") {
        let cr = crate::decode::record_field_payload(rec, "CRID");
        let ptas: Vec<&[u8]> = rec
            .field_tags
            .iter()
            .zip(rec.data_fields.iter())
            .filter(|(t, _)| *t == "PTAS")
            .map(|(_, df)| df.user_data())
            .collect();
        let segh = rec
            .field_tags
            .iter()
            .zip(rec.data_fields.iter())
            .find(|(t, _)| *t == "SEGH")
            .map(|(_, df)| df.user_data());
        let c2il: Vec<&[u8]> = rec
            .field_tags
            .iter()
            .zip(rec.data_fields.iter())
            .filter(|(t, _)| *t == "C2IL")
            .map(|(_, df)| df.user_data())
            .collect();
        if let Some(c) = cr {
            if let Some(cv) = CurveRecord::parse_multi(record_index, c, &ptas, segh, &c2il) {
                return Record::Curve(cv);
            }
        }
    }
    if tags.iter().any(|t| t == "PRID") {
        let pr = crate::decode::record_field_payload(rec, "PRID");
        let c2 = crate::decode::record_field_payload(rec, "C2IT");
        if let (Some(p), Some(c)) = (pr, c2) {
            if let Some(pt) = PointRecord::parse(record_index, p, c) {
                return Record::Point(pt);
            }
        }
    }
    if tags.iter().any(|t| t == "MRID") {
        let mr = crate::decode::record_field_payload(rec, "MRID");
        let mut c3: Vec<u8> = Vec::new();
        for (t, df) in rec.field_tags.iter().zip(rec.data_fields.iter()) {
            if t == "C3IL" {
                let chunk = crate::binary::trim_field_term(df.user_data());
                c3.extend_from_slice(chunk);
            }
        }
        if let Some(m) = mr {
            if !c3.is_empty()
                && let Some(mr) = MridRecord::parse(record_index, m, &c3)
            {
                return Record::Mrid(mr);
            }
        }
    }
    Record::Unknown { record_index, tags }
}

//! Feature **geometry graph**: resolve `FRID` + `SPAS` against typed spatial records and FC.
//!
//! **Curve** lookups accept **RIAS** / **CUCO** references with `rrn == 125` when the matching
//! **CRID** record uses `rcnm == 120` (IHO **S-64 v1.2.0** quirk). **MRID** + **C3IL** rows are
//! indexed for multipoint `SPAS` references (`rrn == 115`).
use std::collections::HashMap;

use s_100::{Curve2D, FeatureObjectId, Geometry, MultiPoint2D, Point2D, Surface2D};

use crate::S101Error;
use crate::binary::parse_ftcs;
use crate::dataset::S101Dataset;
use crate::fc::{FeatureCatalogue, FeatureType, SimpleAttribute};
use crate::geometry::IntegerCrsParameters;
use crate::record::SpasRef;
use crate::record::{
    CompositeCurveRecord, CurveRecord, FeatureRecord, MridRecord, PointRecord, Record,
    SurfaceRecord,
};

/// Resolved feature with FC-backed class + attributes and WGS84 geometry.
#[derive(Debug)]
pub struct Feature<'fc> {
    pub foid: FeatureObjectId,
    pub class: Option<&'fc FeatureType>,
    /// Number of **ATTR** tuples carried on the source ISO 8211 feature record.
    pub attr_source_count: usize,
    /// **ATTR** tuples whose `ATIX` does not map to a **simple** attribute in the supplied FC
    /// (e.g. complex attributes, or catalogue edition skew).
    pub skipped_attr_tuples: Vec<(u16, Vec<u8>)>,
    /// Number of **SPAS** fields on the source feature record.
    pub spatial_assoc_count: usize,
    pub attributes: Vec<ResolvedAttribute<'fc>>,
    pub geometry: Geometry,
}

/// Attribute value resolved through the feature catalogue.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Text(String),
    Enumeration { code: u32, label: Option<String> },
    Raw(Vec<u8>),
}

#[derive(Debug)]
pub struct ResolvedAttribute<'fc> {
    pub fc_entry: &'fc SimpleAttribute,
    pub value: AttributeValue,
}

/// All resolved features in one dataset.
#[derive(Debug)]
pub struct FeatureGraph<'fc> {
    pub features: Vec<Feature<'fc>>,
}

impl S101Dataset {
    /// Build a [`FeatureGraph`] using catalogue-driven attribute typing.
    pub fn build_feature_graph<'fc>(
        &self,
        fc: &'fc FeatureCatalogue,
    ) -> Result<FeatureGraph<'fc>, S101Error> {
        build_feature_graph_inner(self, fc)
    }
}

fn build_feature_graph_inner<'fc>(
    dataset: &S101Dataset,
    fc: &'fc FeatureCatalogue,
) -> Result<FeatureGraph<'fc>, S101Error> {
    let records = dataset.typed_records();
    let crs = dataset.integer_crs_parameters().unwrap_or_default();

    let mut ftcs_rows: Vec<(String, u16)> = Vec::new();
    for r in &records {
        if let Record::DatasetDescription(ds) = r {
            if let Some(ref raw) = ds.ftcs {
                ftcs_rows = parse_ftcs(raw);
            }
            break;
        }
    }

    let mut points: HashMap<(u8, u32), PointRecord> = HashMap::new();
    let mut mrids: HashMap<(u8, u32), MridRecord> = HashMap::new();
    let mut curves: HashMap<(u8, u32), CurveRecord> = HashMap::new();
    let mut surfaces: HashMap<(u8, u32), SurfaceRecord> = HashMap::new();
    let mut composites: HashMap<(u8, u32), CompositeCurveRecord> = HashMap::new();

    for r in &records {
        match r {
            Record::Point(p) => {
                points.insert((p.id.rcnm, p.id.rcid), p.clone());
            }
            Record::Mrid(m) => {
                mrids.insert((m.id.rcnm, m.id.rcid), m.clone());
            }
            Record::Curve(c) => {
                curves.insert((c.id.rcnm, c.id.rcid), c.clone());
            }
            Record::Surface(s) => {
                surfaces.insert((s.id.rcnm, s.id.rcid), s.clone());
            }
            Record::CompositeCurve(cc) => {
                composites.insert((cc.id.rcnm, cc.id.rcid), cc.clone());
            }
            _ => {}
        }
    }

    let mut features_out = Vec::new();
    for r in records {
        let Record::Feature(f) = r else {
            continue;
        };
        let class = resolve_feature_class(&ftcs_rows, fc, &f);
        let (attributes, skipped_attr_tuples) = resolve_attributes(fc, &f);
        let geometry =
            resolve_feature_geometry(&crs, &f, &points, &mrids, &curves, &surfaces, &composites);
        features_out.push(Feature {
            foid: f.foid,
            class,
            attr_source_count: f.attributes.len(),
            skipped_attr_tuples,
            spatial_assoc_count: f.spatial.len(),
            attributes,
            geometry,
        });
    }

    Ok(FeatureGraph {
        features: features_out,
    })
}

fn resolve_feature_class<'fc>(
    ftcs: &[(String, u16)],
    fc: &'fc FeatureCatalogue,
    f: &FeatureRecord,
) -> Option<&'fc FeatureType> {
    let nftc = f.frid.nftc;
    for (name, code) in ftcs {
        if *code == nftc {
            return fc.feature_type_by_code(name);
        }
    }
    None
}

fn resolve_attributes<'fc>(
    fc: &'fc FeatureCatalogue,
    f: &FeatureRecord,
) -> (Vec<ResolvedAttribute<'fc>>, Vec<(u16, Vec<u8>)>) {
    let mut out = Vec::new();
    let mut skipped = Vec::new();
    for a in &f.attributes {
        let Some(sa) = fc.simple_attr_by_source_id(a.atix) else {
            skipped.push((a.atix, a.atvl.clone()));
            continue;
        };
        let value = decode_attribute_value(sa, &a.atvl);
        out.push(ResolvedAttribute {
            fc_entry: sa,
            value,
        });
    }
    (out, skipped)
}

fn decode_attribute_value(sa: &SimpleAttribute, raw: &[u8]) -> AttributeValue {
    let vt = sa.value_type.to_ascii_lowercase();
    if raw.len() == 1 && matches!(vt.as_str(), "boolean" | "integer") {
        return AttributeValue::Integer(i64::from(raw[0] as i8));
    }
    let s = String::from_utf8_lossy(raw).trim().to_string();
    match vt.as_str() {
        "boolean" => {
            let b = matches!(s.to_ascii_lowercase().as_str(), "true" | "1" | "yes");
            AttributeValue::Boolean(b)
        }
        "integer" | "uint32" | "s100_integer" => s
            .parse::<i64>()
            .map(AttributeValue::Integer)
            .unwrap_or_else(|_| AttributeValue::Raw(raw.to_vec())),
        "real" | "double" | "float" => s
            .parse::<f64>()
            .map(AttributeValue::Real)
            .unwrap_or_else(|_| AttributeValue::Raw(raw.to_vec())),
        "enumeration" | "list" => {
            let code: u32 = s.parse().unwrap_or(0);
            let label =
                sa.listed_values.iter().find(|lv| lv.code == code).map(|lv| lv.label.clone());
            AttributeValue::Enumeration { code, label }
        }
        "text" | "characterstring" | "string" => AttributeValue::Text(s),
        _ => {
            if raw.len() == 1 {
                AttributeValue::Integer(i64::from(raw[0] as i8))
            } else {
                AttributeValue::Raw(raw.to_vec())
            }
        }
    }
}

fn resolve_feature_geometry(
    crs: &IntegerCrsParameters,
    f: &FeatureRecord,
    points: &HashMap<(u8, u32), PointRecord>,
    mrids: &HashMap<(u8, u32), MridRecord>,
    curves: &HashMap<(u8, u32), CurveRecord>,
    surfaces: &HashMap<(u8, u32), SurfaceRecord>,
    composites: &HashMap<(u8, u32), CompositeCurveRecord>,
) -> Geometry {
    let mut pts: Vec<Point2D> = Vec::new();
    let mut curves_acc: Vec<Vec<Point2D>> = Vec::new();
    let mut one_surface: Option<Surface2D> = None;

    for s in &f.spatial {
        if let Some(g) =
            spas_to_geometry(crs, s, points, mrids, curves, surfaces, composites, curves)
        {
            match g {
                Geometry::Point(p) => pts.push(p),
                Geometry::Curve(c) => curves_acc.push(c.vertices),
                Geometry::Surface(su) => one_surface = Some(su),
                Geometry::MultiPoint(m) => pts.extend(m.0),
            }
        }
    }

    if let Some(su) = one_surface {
        return Geometry::Surface(su);
    }
    if curves_acc.len() == 1 {
        return Geometry::Curve(Curve2D::new(curves_acc.swap_remove(0)));
    }
    if curves_acc.len() > 1 {
        let mut flat = Vec::new();
        for c in curves_acc {
            flat.extend(c);
        }
        return Geometry::Curve(Curve2D::new(flat));
    }
    if pts.len() == 1 {
        return Geometry::Point(pts[0]);
    }
    if pts.len() > 1 {
        return Geometry::MultiPoint(MultiPoint2D(pts));
    }
    Geometry::Curve(Curve2D::new(Vec::new()))
}

#[allow(clippy::too_many_arguments)] // spatial lookup maps stay explicit per kind
fn spas_to_geometry(
    crs: &IntegerCrsParameters,
    s: &SpasRef,
    points: &HashMap<(u8, u32), PointRecord>,
    mrids: &HashMap<(u8, u32), MridRecord>,
    curves: &HashMap<(u8, u32), CurveRecord>,
    surfaces: &HashMap<(u8, u32), SurfaceRecord>,
    composites: &HashMap<(u8, u32), CompositeCurveRecord>,
    curves_for_comp: &HashMap<(u8, u32), CurveRecord>,
) -> Option<Geometry> {
    let k = (s.rrn, s.rrid);
    if let Some(p) = points.get(&k) {
        return Some(Geometry::Point(xy_to_point(crs, p.ycoo, p.xcoo)));
    }
    if let Some(m) = mrids.get(&k) {
        let pts: Vec<Point2D> = m.yx_pairs.iter().map(|(y, x)| xy_to_point(crs, *y, *x)).collect();
        if pts.is_empty() {
            return None;
        }
        if pts.len() == 1 {
            return Some(Geometry::Point(pts[0]));
        }
        return Some(Geometry::MultiPoint(MultiPoint2D(pts)));
    }
    if let Some(c) = resolve_curve_ref(curves, k.0, k.1) {
        let v = curve_vertices_wgs84(crs, c);
        if v.len() >= 2 {
            return Some(Geometry::Curve(Curve2D::new(v)));
        }
    }
    if let Some(su) = surfaces.get(&k) {
        return surface_to_geometry(crs, su, curves);
    }
    if let Some(cc) = composites.get(&k) {
        let mut verts: Vec<Point2D> = Vec::new();
        for comp in &cc.components {
            if let Some(c) = resolve_curve_ref(curves_for_comp, comp.rrn, comp.rrid) {
                verts.extend(curve_vertices_wgs84(crs, c));
            }
        }
        if verts.len() >= 2 {
            return Some(Geometry::Curve(Curve2D::new(verts)));
        }
    }
    None
}

fn xy_to_point(crs: &IntegerCrsParameters, y: i32, x: i32) -> Point2D {
    let lat = crs.dco_y + f64::from(y) / f64::from(crs.cmf_y);
    let lon = crs.dco_x + f64::from(x) / f64::from(crs.cmf_x);
    Point2D::new(lat, lon)
}

fn resolve_curve_ref(
    curves: &HashMap<(u8, u32), CurveRecord>,
    rrn: u8,
    rrid: u32,
) -> Option<&CurveRecord> {
    if let Some(c) = curves.get(&(rrn, rrid)) {
        return Some(c);
    }
    // S-64 v1.2.0 quirk: **RIAS** / **CUCO** sometimes use `rrn == 125` while **CRID** rows carry
    // `rcnm == 120` for the same `rcid` (curve spatial records).
    if rrn != 120 {
        return curves.get(&(120, rrid));
    }
    None
}

fn curve_vertices_wgs84(crs: &IntegerCrsParameters, c: &CurveRecord) -> Vec<Point2D> {
    c.c2il_vertices.iter().map(|(y, x)| xy_to_point(crs, *y, *x)).collect()
}

fn surface_to_geometry(
    crs: &IntegerCrsParameters,
    su: &SurfaceRecord,
    curves: &HashMap<(u8, u32), CurveRecord>,
) -> Option<Geometry> {
    let mut rings: Vec<Vec<Point2D>> = Vec::new();
    for ring in &su.rings {
        let Some(c) = resolve_curve_ref(curves, ring.rrn, ring.rrid) else {
            continue;
        };
        let mut v = curve_vertices_wgs84(crs, c);
        if ring.ornt == 2 {
            v.reverse();
        }
        rings.push(v);
    }
    if rings.is_empty() || rings[0].len() < 2 {
        return None;
    }
    let exterior = Curve2D::new(rings.remove(0));
    let interiors: Vec<Curve2D> = rings.into_iter().map(Curve2D::new).collect();
    Some(Geometry::Surface(Surface2D::new(exterior, interiors)))
}

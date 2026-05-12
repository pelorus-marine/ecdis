//! Render **S-101 [`FeatureGraph`]** geometry (FC-resolved) to a flat SVG in the same pixel space as
//! [`chart_preview_svg`](chart_preview_svg.rs) / `ecdis-ui`.
//!
//! ```text
//! cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- \
//!   /real/path/to/cell.000 /real/path/to/S-101_FC.xml chart_from_graph.svg
//! ```
//!
//! (`path/to/...` in the README is illustrative only — both inputs must exist on disk.)
//!
//! Optional 4th argument: scale denominator (default `22000`, same order of magnitude as the UI stub).

use std::fmt::Write;
use std::path::Path;

use ecdis_portrayal::{ChartViewportState, UI_CHART_VIEWBOX_HEIGHT_PX, UI_CHART_VIEWBOX_WIDTH_PX};
use s_100::{Curve2D, Geometry, MultiPoint2D, Point2D, Surface2D};
use s_101::{FeatureCatalogue, S101Dataset};

const MAX_SEGMENTS: usize = 120_000;

fn usage() -> &'static str {
    "usage: cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- \\\n\
     <cell.000> <S-101_FC.xml> <out.svg> [scale_denom]\n\
     \n\
     Example (S-64 cache after one corpus fetch):\n\
       unzip -p ~/.cache/pelorus-marine/s-164/S-64_1.2.0.zip \\\n\
         'S-100/DisplayStandard/S100_ROOT/S-101/DATASET_FILES/10100AA_STNDR.000' > /tmp/stndr.000\n\
       unzip -p ~/.cache/pelorus-marine/s-164/S-64_1.2.0.zip \\\n\
         'S-100/InitialCatalogues/S100_ROOT/S-101/CATALOGUES/S-101_1.0.2_20220524.xml' > /tmp/s101_fc.xml\n\
       cargo run -p ecdis-portrayal --example feature_graph_preview_svg -- \\\n\
         /tmp/stndr.000 /tmp/s101_fc.xml /tmp/graph.svg"
}

fn check_input_path(label: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if path.contains("path/to/") {
        return Err(format!(
            "{label}: `{path}` looks like a documentation placeholder, not a real file.\n{}",
            usage()
        )
        .into());
    }
    let p = Path::new(path);
    if !p.is_file() {
        return Err(format!(
            "{label}: no such file (or not a regular file): {path}\n{}",
            usage()
        )
        .into());
    }
    Ok(())
}

fn esc_xml(t: &str) -> String {
    t.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Collect `(lat°, lon°)` samples for framing (same semantics as [`CpuOutlinePortrayal`](ecdis_portrayal::CpuOutlinePortrayal) chains).
fn push_geometry_samples(g: &Geometry, out: &mut Vec<(f64, f64)>) {
    match g {
        Geometry::Point(p) => out.push((p.lat_deg, p.lon_deg)),
        Geometry::MultiPoint(MultiPoint2D(pts)) => {
            for p in pts {
                out.push((p.lat_deg, p.lon_deg));
            }
        }
        Geometry::Curve(c) => {
            for v in &c.vertices {
                out.push((v.lat_deg, v.lon_deg));
            }
        }
        Geometry::Surface(s) => {
            for v in &s.exterior.vertices {
                out.push((v.lat_deg, v.lon_deg));
            }
            for ring in &s.interiors {
                for v in &ring.vertices {
                    out.push((v.lat_deg, v.lon_deg));
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct PixelTx {
    scale: f64,
    cos_vp: f64,
    w: f64,
    h: f64,
}

fn pixel_transform(
    pts: &[(f64, f64)],
    vp: &ChartViewportState,
    width_px: f32,
    height_px: f32,
) -> Option<PixelTx> {
    if pts.is_empty() {
        return None;
    }
    let w = f64::from(width_px.max(32.0));
    let h = f64::from(height_px.max(32.0));

    let mut mn_lat = f64::MAX;
    let mut mx_lat = f64::MIN;
    let mut mn_lon = f64::MAX;
    let mut mx_lon = f64::MIN;
    for (lat, lon) in pts {
        mn_lat = mn_lat.min(*lat);
        mx_lat = mx_lat.max(*lat);
        mn_lon = mn_lon.min(*lon);
        mx_lon = mx_lon.max(*lon);
    }

    let lat_span = (mx_lat - mn_lat).max(1e-8);
    let lon_span = (mx_lon - mn_lon).max(1e-8);
    let mid_lat = (mn_lat + mx_lat) * 0.5;
    let cos_mid = mid_lat.to_radians().cos().abs().max(0.12);
    let fit_lon = lon_span * cos_mid;
    let fit_scale = (w / fit_lon).min(h / lat_span) * 0.88;
    let denom_zoom =
        (22_000f64 / f64::from(vp.scale_denominator.max(500))).sqrt().clamp(0.35, 6.0);
    let scale = fit_scale * denom_zoom;
    let cos_vp = vp.center_lat_deg.to_radians().cos().abs().max(0.12);

    Some(PixelTx {
        scale,
        cos_vp,
        w,
        h,
    })
}

fn project(t: PixelTx, vp: &ChartViewportState, lat: f64, lon: f64) -> (f32, f32) {
    let x = ((lon - vp.center_lon_deg) * t.scale * t.cos_vp + t.w * 0.5) as f32;
    let y = (t.h * 0.5 - (lat - vp.center_lat_deg) * t.scale) as f32;
    (x, y)
}

fn polyline_d(t: PixelTx, vp: &ChartViewportState, verts: &[Point2D]) -> Option<String> {
    if verts.len() < 2 {
        return None;
    }
    let mut s = String::new();
    let (x0, y0) = project(t, vp, verts[0].lat_deg, verts[0].lon_deg);
    write!(&mut s, "M {x0:.2} {y0:.2}").ok()?;
    for v in verts.iter().skip(1) {
        let (x, y) = project(t, vp, v.lat_deg, v.lon_deg);
        write!(&mut s, " L {x:.2} {y:.2}").ok()?;
    }
    Some(s)
}

struct SegBudget {
    used: usize,
}

impl SegBudget {
    fn take(&mut self, n: usize) -> bool {
        if self.used + n > MAX_SEGMENTS {
            return false;
        }
        self.used += n;
        true
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enc = std::env::args().nth(1).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, usage())
    })?;
    let fc_path = std::env::args().nth(2).ok_or("missing FC xml path")?;
    let out_path = std::env::args().nth(3).ok_or("missing output svg path")?;
    let scale_denom: u32 = std::env::args()
        .nth(4)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(22_000);

    check_input_path("ENC", &enc)?;
    check_input_path("FC XML", &fc_path)?;

    let fc_bytes = std::fs::read(&fc_path)?;
    let fc = FeatureCatalogue::parse_xml(&fc_bytes)?;
    let chart = S101Dataset::load(&enc)?;
    let graph = chart.build_feature_graph(&fc)?;

    let w = UI_CHART_VIEWBOX_WIDTH_PX;
    let h = UI_CHART_VIEWBOX_HEIGHT_PX;

    let mut samples: Vec<(f64, f64)> = Vec::new();
    for f in &graph.features {
        if !f.geometry.is_empty() {
            push_geometry_samples(&f.geometry, &mut samples);
        }
    }

    let (mid_lat, mid_lon) = if samples.is_empty() {
        (51.0_f64, 2.0_f64)
    } else {
        let mut mn_lat = f64::MAX;
        let mut mx_lat = f64::MIN;
        let mut mn_lon = f64::MAX;
        let mut mx_lon = f64::MIN;
        for (la, lo) in &samples {
            mn_lat = mn_lat.min(*la);
            mx_lat = mx_lat.max(*la);
            mn_lon = mn_lon.min(*lo);
            mx_lon = mx_lon.max(*lo);
        }
        (((mn_lat + mx_lat) * 0.5), ((mn_lon + mx_lon) * 0.5))
    };

    let vp = ChartViewportState {
        center_lat_deg: mid_lat,
        center_lon_deg: mid_lon,
        scale_denominator: scale_denom,
    };

    let sample_count = samples.len();
    let mut frame_pts = samples;
    if frame_pts.is_empty() {
        frame_pts.push((mid_lat, mid_lon));
    }

    let Some(tx) = pixel_transform(&frame_pts, &vp, w, h) else {
        return Err("could not derive pixel transform".into());
    };

    let mut svg = String::new();
    writeln!(
        svg,
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"##,
        w as i32, h as i32, w as i32, h as i32
    )?;
    writeln!(
        svg,
        r##"  <rect width="100%" height="100%" fill="#0e141c"/>"##
    )?;
    writeln!(
        svg,
        r##"  <text x="10" y="18" fill="#aabbcc" font-family="sans-serif" font-size="12">"##
    )?;
    let label = format!(
        "FeatureGraph: {} features (non-empty geom), scale 1:{} — {}",
        graph.features.iter().filter(|f| !f.geometry.is_empty()).count(),
        scale_denom,
        enc
    );
    writeln!(svg, "    {}", esc_xml(&label))?;
    writeln!(svg, r##"  </text>"##)?;

    let mut budget = SegBudget { used: 0 };

    // Surfaces (exterior fill, then strokes)
    for f in &graph.features {
        if let Geometry::Surface(Surface2D { exterior, interiors }) = &f.geometry {
            if exterior.vertices.len() < 2 || !budget.take(exterior.vertices.len()) {
                continue;
            }
            if let Some(d) = polyline_d(tx, &vp, &exterior.vertices) {
                writeln!(
                    svg,
                    r##"  <path d="{d} Z" fill="rgba(76,175,136,0.12)" stroke="#4caf88" stroke-width="1.5" fill-rule="nonzero"/>"##
                )?;
            }
            for ring in interiors {
                if ring.vertices.len() < 2 || !budget.take(ring.vertices.len()) {
                    continue;
                }
                if let Some(d) = polyline_d(tx, &vp, &ring.vertices) {
                    writeln!(
                        svg,
                        r##"  <path d="{d} Z" fill="none" stroke="#2e7d52" stroke-width="1" stroke-dasharray="4 3"/>"##
                    )?;
                }
            }
        }
    }

    for f in &graph.features {
        if let Geometry::Curve(Curve2D { vertices }) = &f.geometry {
            if vertices.len() < 2 || !budget.take(vertices.len()) {
                continue;
            }
            if let Some(d) = polyline_d(tx, &vp, vertices) {
                writeln!(
                    svg,
                    r##"  <path d="{d}" fill="none" stroke="#6ca0d4" stroke-width="1.8"/>"##
                )?;
            }
        }
    }

    for f in &graph.features {
        match &f.geometry {
            Geometry::Point(p) => {
                let (x, y) = project(tx, &vp, p.lat_deg, p.lon_deg);
                writeln!(
                    svg,
                    r##"  <circle cx="{x:.2}" cy="{y:.2}" r="3.5" fill="#ffaa44" stroke="#332200" stroke-width="0.5"/>"##
                )?;
            }
            Geometry::MultiPoint(MultiPoint2D(pts)) => {
                for p in pts {
                    let (x, y) = project(tx, &vp, p.lat_deg, p.lon_deg);
                    writeln!(
                        svg,
                        r##"  <circle cx="{x:.2}" cy="{y:.2}" r="2" fill="#c9a8ff" stroke="#221833" stroke-width="0.4"/>"##
                    )?;
                }
            }
            _ => {}
        }
    }

    writeln!(svg, "</svg>")?;
    std::fs::write(&out_path, &svg)?;
    println!(
        "Wrote {out_path} ({} features in graph, {} sample points for framing)",
        graph.features.len(),
        sample_count
    );
    Ok(())
}

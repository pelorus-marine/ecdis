//! Feature-graph geometry → [`PortrayalLayers`](crate::frame::PortrayalLayers).

use s_100::{Curve2D, Geometry, MultiPoint2D, Point2D, Surface2D};
use s_101::FeatureGraph;

use crate::chart_theme::ChartTheme;
use crate::chart_viewport::ChartViewportState;
use crate::cpu_outline::CpuOutlinePortrayal;
use crate::frame::{FilledPolygon, LineSegment, PointMarker, PortrayalLayers};

const MAX_SEGMENTS: usize = 120_000;

#[derive(Clone, Copy)]
struct PixelTx {
    scale: f64,
    cos_vp: f64,
    w: f64,
    h: f64,
}

enum Projector<'a> {
    /// Same WGS84→pixel mapping as ENC **C2IL** chart outlines.
    C2il(&'a CpuOutlinePortrayal),
    /// Auto-fit to feature geometry (fallback when the cell has no C2IL chains).
    Affine(PixelTx),
}

pub fn layers_from_graph(
    graph: &FeatureGraph<'_>,
    vp: &ChartViewportState,
    width_px: f32,
    height_px: f32,
    theme: &ChartTheme,
    outline: Option<&CpuOutlinePortrayal>,
) -> PortrayalLayers {
    let mut samples = Vec::new();
    for f in &graph.features {
        if !f.geometry.is_empty() {
            push_geometry_samples(&f.geometry, &mut samples);
        }
    }

    let projector = if let Some(o) = outline.filter(|o| o.chain_count() > 0) {
        Projector::C2il(o)
    } else {
        let mut frame_pts = samples.clone();
        if frame_pts.is_empty() {
            frame_pts.push((vp.center_lat_deg, vp.center_lon_deg));
        }
        let Some(tx) = pixel_transform(&frame_pts, vp, width_px, height_px) else {
            return PortrayalLayers {
                caption: "Feature graph — could not derive pixel transform".to_string(),
                ..Default::default()
            };
        };
        Projector::Affine(tx)
    };

    let geo_count = graph.features.iter().filter(|f| !f.geometry.is_empty()).count();
    let projection_note = match projector {
        Projector::C2il(_) => "C2IL-aligned projection",
        Projector::Affine(_) => "auto-fit to feature bbox (no C2IL in cell)",
    };

    let mut layers = PortrayalLayers {
        caption: format!(
            "Feature graph — {} features ({geo_count} with geometry), scale 1:{}, {projection_note}",
            graph.features.len(),
            vp.scale_denominator
        ),
        ..Default::default()
    };

    let mut budget = SegBudget { used: 0 };

    for f in &graph.features {
        if let Geometry::Surface(Surface2D {
            exterior,
            interiors,
        }) = &f.geometry
        {
            if exterior.vertices.len() >= 2 && budget.take(exterior.vertices.len()) {
                if let Some(verts) =
                    polyline_verts(&projector, vp, width_px, height_px, &exterior.vertices)
                {
                    layers.polygons.push(FilledPolygon {
                        vertices: verts,
                        fill: theme.surface_fill,
                        stroke: theme.surface_stroke,
                        fill_alpha: 0.12,
                    });
                }
            }
            for ring in interiors {
                if ring.vertices.len() >= 2 && budget.take(ring.vertices.len()) {
                    if let Some(verts) =
                        polyline_verts(&projector, vp, width_px, height_px, &ring.vertices)
                    {
                        layers.polygons.push(FilledPolygon {
                            vertices: verts,
                            fill: theme.background,
                            stroke: theme.surface_stroke,
                            fill_alpha: 0.0,
                        });
                    }
                }
            }
        }
    }

    for f in &graph.features {
        if let Geometry::Curve(Curve2D { vertices }) = &f.geometry {
            if vertices.len() >= 2 && budget.take(vertices.len()) {
                if let Some(verts) = polyline_verts(&projector, vp, width_px, height_px, vertices) {
                    for w in verts.windows(2) {
                        layers.segments.push(LineSegment {
                            x1: w[0].0,
                            y1: w[0].1,
                            x2: w[1].0,
                            y2: w[1].1,
                            stroke: theme.chart_stroke,
                            width_px: 1.5,
                        });
                    }
                }
            }
        }
    }

    for f in &graph.features {
        match &f.geometry {
            Geometry::Point(p) => {
                if let Some((x, y)) =
                    project_point(&projector, vp, width_px, height_px, p.lat_deg, p.lon_deg)
                {
                    layers.points.push(PointMarker {
                        x,
                        y,
                        radius_px: 3.5,
                        fill: theme.point_fill,
                        stroke: theme.hud_primary,
                    });
                }
            }
            Geometry::MultiPoint(MultiPoint2D(pts)) => {
                for p in pts {
                    if let Some((x, y)) =
                        project_point(&projector, vp, width_px, height_px, p.lat_deg, p.lon_deg)
                    {
                        layers.points.push(PointMarker {
                            x,
                            y,
                            radius_px: 2.0,
                            fill: theme.heading,
                            stroke: theme.hud_primary,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    layers
}

fn project_point(
    projector: &Projector<'_>,
    vp: &ChartViewportState,
    width_px: f32,
    height_px: f32,
    lat: f64,
    lon: f64,
) -> Option<(f32, f32)> {
    match *projector {
        Projector::C2il(outline) => outline.project_wgs84_to_screen_px(vp, lat, lon, width_px, height_px),
        Projector::Affine(tx) => Some(affine_project(tx, vp, lat, lon)),
    }
}

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
    let denom_zoom = (22_000f64 / f64::from(vp.scale_denominator.max(500))).sqrt().clamp(0.35, 6.0);
    let scale = fit_scale * denom_zoom;
    let cos_vp = vp.center_lat_deg.to_radians().cos().abs().max(0.12);

    Some(PixelTx {
        scale,
        cos_vp,
        w,
        h,
    })
}

fn affine_project(t: PixelTx, vp: &ChartViewportState, lat: f64, lon: f64) -> (f32, f32) {
    let x = ((lon - vp.center_lon_deg) * t.scale * t.cos_vp + t.w * 0.5) as f32;
    let y = (t.h * 0.5 - (lat - vp.center_lat_deg) * t.scale) as f32;
    (x, y)
}

fn polyline_verts(
    projector: &Projector<'_>,
    vp: &ChartViewportState,
    width_px: f32,
    height_px: f32,
    verts: &[Point2D],
) -> Option<Vec<(f32, f32)>> {
    if verts.len() < 2 {
        return None;
    }
    Some(
        verts
            .iter()
            .filter_map(|v| {
                project_point(projector, vp, width_px, height_px, v.lat_deg, v.lon_deg)
            })
            .collect(),
    )
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

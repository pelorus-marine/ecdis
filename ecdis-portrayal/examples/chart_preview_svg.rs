//! Render ENC **C2IL** outlines to a flat SVG (same pixel space as `ecdis-ui`).
//!
//! ```text
//! cargo run -p ecdis-portrayal --example chart_preview_svg -- target/iho-cache/sample_enc.000 target/chart_preview.svg
//! ```

use std::fmt::Write;

use ecdis_portrayal::{
    ChartViewportState, CpuOutlinePortrayal, PortrayalPipeline, UI_CHART_VIEWBOX_HEIGHT_PX,
    UI_CHART_VIEWBOX_WIDTH_PX,
};
use s_101::S101Dataset;

fn esc_xml(t: &str) -> String {
    t.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let enc = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/iho-cache/sample_enc.000".into());
    let out_path = std::env::args().nth(2).unwrap_or_else(|| "chart_preview.svg".into());

    let chart = S101Dataset::load(&enc)?;
    let mut p = CpuOutlinePortrayal::default();
    p.reset_for_chart(&chart)?;

    let vp = ChartViewportState {
        center_lat_deg: p.anchor_lat_deg,
        center_lon_deg: p.anchor_lon_deg,
        scale_denominator: 22_000,
    };

    let w = UI_CHART_VIEWBOX_WIDTH_PX;
    let h = UI_CHART_VIEWBOX_HEIGHT_PX;
    let segs = p.segments_screen_px(&vp, w, h);

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
    let label = if segs.is_empty() {
        format!("No C2IL segments (stub cell?) — {}", enc)
    } else {
        format!(
            "{} C2IL segments, {} chains — {}",
            segs.len(),
            p.chain_count(),
            enc
        )
    };
    writeln!(svg, "    {}", esc_xml(&label))?;
    writeln!(svg, r##"  </text>"##)?;

    for (x1, y1, x2, y2) in &segs {
        writeln!(
            svg,
            r##"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#6ca0d4" stroke-width="2"/>"##,
            x1, y1, x2, y2
        )?;
    }

    if let Some((ox, oy)) =
        p.project_wgs84_to_screen_px(&vp, p.anchor_lat_deg, p.anchor_lon_deg, w, h)
    {
        writeln!(
            svg,
            r##"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#ffaa44" stroke-width="3"/>"##,
            ox - 10.0,
            oy,
            ox + 10.0,
            oy
        )?;
        writeln!(
            svg,
            r##"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#ffaa44" stroke-width="3"/>"##,
            ox,
            oy - 10.0,
            ox,
            oy + 10.0
        )?;
    }

    writeln!(svg, "</svg>")?;
    std::fs::write(&out_path, &svg)?;
    println!(
        "Wrote {out_path} ({} line segments, chains={})",
        segs.len(),
        p.chain_count()
    );
    Ok(())
}

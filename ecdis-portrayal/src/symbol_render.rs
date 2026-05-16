//! Rasterize portrayal-catalogue SVG symbols with palette stylesheets.

use s_101::{stylesheet_from_palette, PortrayalCatalogueBundle, PortrayalCatalogueError};

use crate::display_mode::DisplayMode;

/// RGBA bitmap from a symbol render.
#[derive(Debug, Clone)]
pub struct RasterizedSymbol {
    pub width_px: u32,
    pub height_px: u32,
    pub rgba: Vec<u8>,
}

/// Rasterize one catalogue symbol for the given display mode.
pub fn rasterize_symbol(
    bundle: &PortrayalCatalogueBundle,
    symbol_id: &str,
    mode: DisplayMode,
    max_edge_px: u32,
) -> Result<RasterizedSymbol, PortrayalCatalogueError> {
    let svg_bytes = bundle.read_symbol_svg(symbol_id)?;
    let palette = bundle
        .catalogue
        .palette(mode.palette_name())
        .ok_or_else(|| {
            PortrayalCatalogueError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("palette {} not in catalogue", mode.palette_name()),
            ))
        })?;

    let css_bytes = bundle
        .read_palette_stylesheet(palette)
        .unwrap_or_else(|_| stylesheet_from_palette(palette));

    let svg_str = std::str::from_utf8(&svg_bytes).map_err(|_| {
        PortrayalCatalogueError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "symbol SVG is not UTF-8",
        ))
    })?;
    let css_str = std::str::from_utf8(&css_bytes).map_err(|_| {
        PortrayalCatalogueError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "stylesheet is not UTF-8",
        ))
    })?;

    let svg_with_css = prepare_svg_for_render(svg_str, css_str);

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_with_css, &opt).map_err(|e| {
        PortrayalCatalogueError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            e.to_string(),
        ))
    })?;

    let size = tree.size();
    let scale = (f64::from(max_edge_px) / f64::from(size.width().max(size.height()) as f32))
        .min(4.0)
        .max(0.25);
    let w = (f64::from(size.width()) * scale).ceil().max(1.0) as u32;
    let h = (f64::from(size.height()) * scale).ceil().max(1.0) as u32;

    let mut pixmap = tiny_skia::Pixmap::new(w, h).ok_or_else(|| {
        PortrayalCatalogueError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to allocate pixmap",
        ))
    })?;

    let transform = tiny_skia::Transform::from_scale(scale as f32, scale as f32);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Ok(RasterizedSymbol {
        width_px: w,
        height_px: h,
        rgba: pixmap.data().to_vec(),
    })
}

/// Strip external stylesheet PI and embed symbol CSS for `usvg`.
fn prepare_svg_for_render(svg: &str, css: &str) -> String {
    // IHO symbol SVG often places `<?xml-stylesheet …?>` on the same line as `<svg>`; do not
    // drop the whole line or the root element disappears.
    let without_pi = strip_xml_stylesheet_pi(svg);
    let style_block = format!("<style type=\"text/css\"><![CDATA[\n{css}\n]]></style>");
    if let Some(start) = without_pi.find("<svg") {
        if let Some(end) = without_pi[start..].find('>') {
            let insert = start + end + 1;
            return format!(
                "{}\n{style_block}\n{}",
                &without_pi[..insert],
                &without_pi[insert..]
            );
        }
    }
    format!("<svg>{style_block}{without_pi}</svg>")
}

fn strip_xml_stylesheet_pi(svg: &str) -> String {
    let mut out = svg.to_string();
    while let Some(start) = out.find("<?xml-stylesheet") {
        let Some(rel_end) = out[start..].find("?>") else {
            break;
        };
        let end = start + rel_end + 2;
        out.replace_range(start..end, "");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_pi_keeps_svg_root_on_same_line() {
        let svg = r#"<?xml version="1.0"?>
<?xml-stylesheet href="daySvgStyle.css" type="text/css"?><svg width="1"><circle r="1"/></svg>"#;
        let stripped = strip_xml_stylesheet_pi(svg);
        assert!(stripped.contains("<svg"));
        assert!(!stripped.contains("xml-stylesheet"));
        let prepared = prepare_svg_for_render(svg, ".layout{display:none}");
        assert!(prepared.contains("<style"));
        assert_eq!(prepared.matches("<?xml").count(), 1);
    }

    #[test]
    #[ignore = "requires IHO_TESTDATA_ZIP (S-64 corpus zip)"]
    fn achare02_renders_non_trivial_bitmap() {
        use crate::open_s101_portrayal_from_s64_zip;

        let zip = std::env::var_os("IHO_TESTDATA_ZIP").expect("IHO_TESTDATA_ZIP");
        let bundle = open_s101_portrayal_from_s64_zip(&zip).expect("open portrayal from S-64");
        let img = rasterize_symbol(&bundle, "ACHARE02", DisplayMode::Day, 128).expect("render");
        let opaque: u32 = img
            .rgba
            .chunks_exact(4)
            .filter(|p| p[3] > 32)
            .count() as u32;
        assert!(opaque > 50, "expected painted pixels, got {opaque}");
    }
}
